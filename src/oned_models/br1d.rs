use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct BR1D {
    x: Vec<f64>,
    // parameters
    gna: f64,
    gnac: f64,
    ena: f64,
    gs: f64,
    cm: f64,
    // numerical parameters
    dt: f64,
    dx: f64,
    diff: f64,
    outputevery: usize,
    nx: usize,
    period: f64,
    // initial conditions
    v: Vec<f64>,
    m: Vec<f64>,
    h: Vec<f64>,
    j: Vec<f64>,
    d: Vec<f64>,
    f: Vec<f64>,
    x1: Vec<f64>,
    cai: Vec<f64>,
    // extra variables
    boundary: usize,
    // iteration counter
    i: usize,
}

impl BR1D {
    pub fn a_and_b_parts(
        &self,
        c1: f64,
        c2: f64,
        c3: f64,
        c4: f64,
        c5: f64,
        c6: f64,
        c7: f64,
        arr: Vec<f64>,
    ) -> Vec<f64> {
        arr.iter()
            .map(|v| (c1 * (c2 * (v + c3)).exp() + c4 * (v + c5)) / ((c6 * (v + c3)).exp() + c7))
            .collect()
    }

    pub fn diff_vectors(&self, a: Vec<f64>, b: Vec<f64>, arr: Vec<f64>) -> Vec<f64> {
        a.iter()
            .zip(arr.iter())
            .zip(b.iter())
            .map(|((a, arr), b)| a * (1.0 - arr) - b * arr)
            .collect()
    }

    pub fn iterate(&self, arr: Vec<f64>, dt: f64, d_arr: Vec<f64>) -> Vec<f64> {
        arr.iter()
            .zip(d_arr.iter())
            .map(|(arr, d_arr)| arr + dt * d_arr)
            .collect()
    }

    pub fn matrix_vector_smart(&self, vec: Vec<f64>, nx: usize, boundary: usize) -> Vec<f64> {
        let mut temp: Vec<f64> = vec![0.0; nx+2];
        for i in 0..nx {
            temp[i+1] = vec[i];
        }
        let mut result: Vec<f64> = vec![0.0; nx];
        for i in 0..nx {
            result[i] = temp[i] - 2.0 * temp[i+1] + temp[i+2];
        }
        if boundary == 0 {
            result[0] += temp[2];
            result[nx-1] += temp[nx-1];
        } else {
            result[0] += temp[nx];
            result[nx-1] += temp[1];
        }
        result
    }
}

#[wasm_bindgen]
impl BR1D {
    pub fn test(n: usize) -> BR1D {
        BR1D {
            x: vec![0.0; n],
            gna: 0.0,
            gnac: 0.0,
            ena: 0.0,
            gs: 0.0,
            cm: 0.0,
            dt: 0.0,
            dx: 0.0,
            diff: 0.0,
            outputevery: 0,
            nx: 0,
            period: 0.0,
            v: vec![0.0; n],
            m: vec![0.0; n],
            h: vec![0.0; n],
            j: vec![0.0; n],
            d: vec![0.0; n],
            f: vec![0.0; n],
            x1: vec![0.0; n],
            cai: vec![0.0; n],
            boundary: 0,
            i: 0
        }
    }


    pub fn new(
        v: f64,
        m: f64,
        h: f64,
        j: f64,
        d: f64,
        f: f64,
        x1: f64,
        cai: f64,
        gna: f64,
        gnac: f64,
        ena: f64,
        gs: f64,
        cm: f64,
        dt: f64,
        dx: f64,
        diff: f64,
        outputevery: usize,
        nx: usize,
        period: f64,
        boundary: usize,
    ) -> BR1D {
        let x = (0..nx).map(|i| i as f64 * dx as f64).collect();
        let v = vec![v; nx];
        let m = vec![m; nx];
        let h = vec![h; nx];
        let j = vec![j; nx];
        let d = vec![d; nx];
        let f = vec![f; nx];
        let x1 = vec![x1; nx];
        let cai = vec![cai; nx];

        BR1D {
            x,
            gna,
            gnac,
            ena,
            gs,
            cm,
            dt,
            dx,
            diff,
            outputevery,
            nx,
            period,
            v,
            m,
            h,
            j,
            d,
            f,
            x1,
            cai,
            boundary,
            i: 0
        }
    }

    pub fn draw(&self, canvas: HtmlCanvasElement) {
        // self.clear(canvas.clone());
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        let (x_min, x_max) = (0.0, (self.nx as f64 * self.dx - self.dx) as f64);
        let (y_min, y_max) = (-100.0, 100.0);
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            // .x_label_area_size(40)
            // .y_label_area_size(40)
            // .margin(50)
            // .caption("BR1D", ("Arial", 20).into_font())
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Position (cm)")
            .y_desc("Voltage (mV)")
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                self.x.iter().zip(self.v.iter()).map(|(x, y)| (*x, *y)),
                &RED,
            ))
            .unwrap();
    }

    pub fn tick(&mut self) {
        let mut istim: Vec<f64>;
        let stimmag = 26.4;

        let mut c1: f64;
        let mut c2: f64;
        let mut c3: f64;
        let mut c4: f64;
        let mut c5: f64;
        let mut c6: f64;
        let mut c7: f64;

        let mut ax1: Vec<f64>;
        let mut bx1: Vec<f64>;
        let mut am: Vec<f64>;
        let mut bm: Vec<f64>;
        let mut ah: Vec<f64>;
        let mut bh: Vec<f64>;
        let mut aj: Vec<f64>;
        let mut bj: Vec<f64>;
        let mut ad: Vec<f64>;
        let mut bd: Vec<f64>;
        let mut af: Vec<f64>;
        let mut bf: Vec<f64>;

        let mut dv: Vec<f64>;
        let mut dm: Vec<f64>;
        let mut dh: Vec<f64>;
        let mut dj: Vec<f64>;
        let mut dd: Vec<f64>;
        let mut df: Vec<f64>;
        let mut dx1: Vec<f64>;
        let mut dcai: Vec<f64>;

        let mut es: Vec<f64>;
        let mut ik1: Vec<f64>;
        let mut ix1: Vec<f64>;
        let mut ina: Vec<f64>;
        let mut is: Vec<f64>;
        let mut xlap: Vec<f64>;

        let mut stimtemplate = vec![0.0; self.nx];
        for i in 0..10 {
            stimtemplate[i] = 1.0;
        }

        // for loop from 0 to outputevery
        for ntime in self.i..self.i + self.outputevery {
            c1 = 0.0005;
            c2 = 0.083;
            c3 = 50.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.057;
            c7 = 1.0;
            ax1 = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.0013;
            c2 = -0.06;
            c3 = 20.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.04;
            c7 = 1.0;
            bx1 = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.0;
            c2 = 0.0;
            c3 = 47.0;
            c4 = -1.0;
            c5 = 47.0;
            c6 = -0.1;
            c7 = -1.0;
            am = self
                .v
                .iter()
                .map(|v| {
                    if (v + c3).abs() < 1e-5 {
                        10.0
                    } else {
                        (c1 * (c2 * (v + c3)).exp() + c4 * (v + c5)) / ((c6 * (v + c3)).exp() + c7)
                    }
                })
                .collect();
            c1 = 40.0;
            c2 = -0.056;
            c3 = 72.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.0;
            c7 = 0.0;
            bm = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.126;
            c2 = -0.25;
            c3 = 77.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.0;
            c7 = 0.0;
            ah = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 1.7;
            c2 = 0.0;
            c3 = 22.5;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.082;
            c7 = 1.0;
            bh = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.055;
            c2 = -0.25;
            c3 = 78.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.2;
            c7 = 1.0;
            aj = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.3;
            c2 = 0.0;
            c3 = 32.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.1;
            c7 = 1.0;
            bj = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.095;
            c2 = -0.01;
            c3 = -5.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.072;
            c7 = 1.0;
            ad = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.07;
            c2 = -0.017;
            c3 = 44.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.05;
            c7 = 1.0;
            bd = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.012;
            c2 = -0.008;
            c3 = 28.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.15;
            c7 = 1.0;
            af = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());
            c1 = 0.0065;
            c2 = -0.02;
            c3 = 30.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.2;
            c7 = 1.0;
            bf = self.a_and_b_parts(c1, c2, c3, c4, c5, c6, c7, self.v.clone());

            dm = self.diff_vectors(am, bm, self.m.clone());
            dh = self.diff_vectors(ah, bh, self.h.clone());
            dj = self.diff_vectors(aj, bj, self.j.clone());
            dd = self.diff_vectors(ad, bd, self.d.clone());
            df = self.diff_vectors(af, bf, self.f.clone());
            dx1 = self.diff_vectors(ax1, bx1, self.x1.clone());

            self.m = self.iterate(self.m.clone(), self.dt, dm);
            self.h = self.iterate(self.h.clone(), self.dt, dh);
            self.j = self.iterate(self.j.clone(), self.dt, dj);
            self.d = self.iterate(self.d.clone(), self.dt, dd);
            self.f = self.iterate(self.f.clone(), self.dt, df);
            self.x1 = self.iterate(self.x1.clone(), self.dt, dx1);

            es = self
                .cai
                .iter()
                .map(|cai| -82.3 - 13.0287 * cai.ln())
                .collect();

            ik1 = self
            .v
            .iter()
            .map(|v| {
                0.35 * (4.0 * ((0.04 * (v + 85.0)).exp() - 1.0)
                / ((0.08 * (v + 53.0)).exp() + (0.04 * (v + 53.0)).exp())
                + 0.2 * (v + 23.0) / (1.0 - (-0.04 * (v + 23.0)).exp()))
            })
            .collect();
            
            ix1 = self
                .x1
                .iter()
                .zip(self.v.iter())
                .map(|(x1, v)| {
                    x1 * 0.8 * ((0.04 * (v + 77.0)).exp() - 1.0) / (0.04 * (v + 35.0)).exp()
                })
                .collect();
            
            ina = self
                .m
                .iter()
                .zip(self.h.iter())
                .zip(self.j.iter())
                .map(|((m, h), j)| self.gna * m * m * m * h * j + self.gnac)
                .zip(self.v.iter())
                .map(|(g, v)| g * (v - self.ena))
                .collect();
            
            is = self
                .d
                .iter()
                .zip(self.f.iter())
                .map(|(d, f)| d * f)
                .zip(self.v.iter())
                .zip(es.iter())
                .map(|((df, v), es)| self.gs * df * (v - es))
                .collect();

            dcai = is
                .iter()
                .zip(self.cai.iter())
                .map(|(is, cai)| -1e-7 * is + 0.07 * (1e-7 - cai))
                .collect();

            self.cai = self.iterate(self.cai.clone(), self.dt, dcai);

            istim = vec![0.0; self.nx];
            if ntime % ((self.period / self.dt) as usize) < ((2.0 / self.dt) as usize) {
                istim = stimtemplate.iter().map(|x| stimmag * x).collect();
            }

            dv = ik1
                .iter()
                .zip(ix1.iter())
                .zip(ina.iter())
                .zip(is.iter())
                .zip(istim.iter())
                .map(|((((ik1, ix1), ina), is), istim)| -(ik1 + ix1 + ina + is - istim) / self.cm)
                .collect();

            xlap = self.matrix_vector_smart(self.v.clone(), self.nx, self.boundary);
            // xlap = vec![0.0; self.nx];

            xlap = xlap
                .iter()
                .map(|x| self.diff * self.dt * x / (self.dx * self.dx))
                .collect();

            self.v = self
                .v
                .iter()
                .zip(dv.iter())
                .zip(xlap.iter())
                .map(|((v, dv), xlap)| v + self.dt * dv + xlap)
                .collect();
        }

        self.i += self.outputevery;
    }

    pub fn set_stimulus(&mut self, index: usize, v: f64) {
        self.v[index] = v;
    }

    pub fn set_boundary(&mut self, boundary: usize) {
        self.boundary = boundary;
    }

    pub fn clear(&self, canvas: HtmlCanvasElement) {
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();
    }
}
