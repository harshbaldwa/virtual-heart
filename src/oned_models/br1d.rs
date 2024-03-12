use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct BR1D {
    x: Vec<f64>,
    // parameters
    gna: f32,
    gnac: f32,
    ena: f32,
    gs: f32,
    cm: f32,
    // numerical parameters
    dt: f32,
    dx: f32,
    diff: f32,
    outputevery: usize,
    nx: usize,
    // initial conditions
    v: Vec<f32>,
    m: Vec<f32>,
    h: Vec<f32>,
    j: Vec<f32>,
    d: Vec<f32>,
    f: Vec<f32>,
    x1: Vec<f32>,
    cai: Vec<f32>,
    // extra variables
    boundary: usize,
    // iteration counter
    i: usize,
}

impl BR1D {
    pub fn a_and_b_parts(
        &self,
        c1: f32,
        c2: f32,
        c3: f32,
        c4: f32,
        c5: f32,
        c6: f32,
        c7: f32,
        arr: Vec<f32>,
    ) -> Vec<f32> {
        arr.iter()
            .map(|v| (c1 * (c2 * (v + c3)).exp() + c4 * (v + c5)) / ((c6 * (v + c3)).exp() + c7))
            .collect()
    }

    pub fn diff_vectors(&self, a: Vec<f32>, b: Vec<f32>, arr: Vec<f32>) -> Vec<f32> {
        a.iter()
            .zip(arr.iter())
            .zip(b.iter())
            .map(|((a, arr), b)| a * (1.0 - arr) - b * arr)
            .collect()
    }

    pub fn iterate(&self, arr: Vec<f32>, dt: f32, d_arr: Vec<f32>) -> Vec<f32> {
        arr.iter()
            .zip(d_arr.iter())
            .map(|(arr, d_arr)| arr + dt * d_arr)
            .collect()
    }

    pub fn matrix_vector_smart(&self, vec: Vec<f32>, nx: usize, boundary: usize) -> Vec<f32> {
        let mut temp: Vec<f32> = vec![0.0; nx+2];
        for i in 0..nx {
            temp[i+1] = vec[i];
        }
        let mut result: Vec<f32> = vec![0.0; nx];
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
    pub fn new(
        v: f32,
        m: f32,
        h: f32,
        j: f32,
        d: f32,
        f: f32,
        x1: f32,
        cai: f32,
        gna: f32,
        gnac: f32,
        ena: f32,
        gs: f32,
        cm: f32,
        dt: f32,
        dx: f32,
        diff: f32,
        outputevery: usize,
        nx: usize,
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
            v,
            m,
            h,
            j,
            d,
            f,
            x1,
            cai,
            boundary,
            i: 0,
        }
    }

    pub fn draw(&self, canvas: HtmlCanvasElement) {
        // self.clear(canvas.clone());
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        let (x_min, x_max) = (0.0, (self.nx as f32 * self.dx - self.dx) as f64);
        let (y_min, y_max) = (-100.0, 100.0);
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(50)
            .caption("BR1D", ("Arial", 20).into_font())
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Position (cm)")
            .y_desc("Voltage (mV)")
            .draw()
            .unwrap();

        // convert x and v to f64
        let vf64: Vec<f64> = self.v.iter().map(|v| *v as f64).collect();

        chart
            .draw_series(LineSeries::new(
                self.x.iter().zip(vf64.iter()).map(|(x, y)| (*x, *y)),
                &RED,
            ))
            .unwrap();
    }

    pub fn tick(&mut self) {
        let period = 310.0;
        let mut istim: Vec<f32>;
        let stimmag = 26.4;

        let mut c1: f32;
        let mut c2: f32;
        let mut c3: f32;
        let mut c4: f32;
        let mut c5: f32;
        let mut c6: f32;
        let mut c7: f32;

        let mut ax1: Vec<f32>;
        let mut bx1: Vec<f32>;
        let mut am: Vec<f32>;
        let mut bm: Vec<f32>;
        let mut ah: Vec<f32>;
        let mut bh: Vec<f32>;
        let mut aj: Vec<f32>;
        let mut bj: Vec<f32>;
        let mut ad: Vec<f32>;
        let mut bd: Vec<f32>;
        let mut af: Vec<f32>;
        let mut bf: Vec<f32>;

        let mut dv: Vec<f32>;
        let mut dm: Vec<f32>;
        let mut dh: Vec<f32>;
        let mut dj: Vec<f32>;
        let mut dd: Vec<f32>;
        let mut df: Vec<f32>;
        let mut dx1: Vec<f32>;
        let mut dcai: Vec<f32>;

        let mut es: Vec<f32>;
        let mut ik1: Vec<f32>;
        let mut ix1: Vec<f32>;
        let mut ina: Vec<f32>;
        let mut is: Vec<f32>;
        let mut xlap: Vec<f32>;

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
            if ntime % ((period / self.dt) as usize) < ((2.0 / self.dt) as usize) {
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

    pub fn get_min_v(&self) -> f32 {
        *self.v.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    pub fn clear(&self, canvas: HtmlCanvasElement) {
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();
    }
}
