use js_sys::Math::abs;
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct BR0D {
    // time array
    t: Vec<f64>,
    nsteps: usize,
    // voltage arrays
    vsave: Vec<f64>,
    msave: Vec<f64>,
    hsave: Vec<f64>,
    jsave: Vec<f64>,
    dsave: Vec<f64>,
    fsave: Vec<f64>,
    x1save: Vec<f64>,
    caisave: Vec<f64>,
    // current arrays
    inasave: Vec<f64>,
    issave: Vec<f64>,
    ik1save: Vec<f64>,
    ix1save: Vec<f64>,
}

#[wasm_bindgen]
impl BR0D {
    pub fn new(v: f64, m: f64, h: f64, j: f64, d: f64, f: f64, x1: f64, cai: f64, nsteps: usize) -> BR0D {
        let t = (0..nsteps).map(|x| x as f64 * 0.1).collect();
        let mut vsave = vec![0.0; nsteps];
        let mut msave = vec![0.0; nsteps];
        let mut hsave = vec![0.0; nsteps];
        let mut jsave = vec![0.0; nsteps];
        let mut dsave = vec![0.0; nsteps];
        let mut fsave = vec![0.0; nsteps];
        let mut x1save = vec![0.0; nsteps];
        let mut caisave = vec![0.0; nsteps];

        let inasave = vec![0.0; nsteps];
        let issave = vec![0.0; nsteps];
        let ik1save = vec![0.0; nsteps];
        let ix1save = vec![0.0; nsteps];

        vsave[0] = v;
        msave[0] = m;
        hsave[0] = h;
        jsave[0] = j;
        dsave[0] = d;
        fsave[0] = f;
        x1save[0] = x1;
        caisave[0] = cai;

        BR0D {
            t,
            nsteps,
            vsave,
            msave,
            hsave,
            jsave,
            dsave,
            fsave,
            x1save,
            caisave,
            inasave,
            issave,
            ik1save,
            ix1save,
        }
    }

    pub fn draw(&self, canvas: HtmlCanvasElement) {
        self.clear(canvas.clone());
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        let (x_min, x_max) = (0.0, self.nsteps as f64 * 0.1);
        let (y_min, y_max) = (-100.0, 50.0);
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(55)
            .caption("BR0D", ("Arial", 20).into_font())
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Time (ms)")
            .y_desc("Voltage (mV)")
            .draw()
            .unwrap();

        chart
            .draw_series(LineSeries::new(
                self.t.iter().zip(self.vsave.iter()).map(|(x, y)| (*x, *y)),
                &RED,
            ))
            .unwrap();
    }

    pub fn draw_gates(&self, canvas: HtmlCanvasElement, gates: &str) {
        self.clear(canvas.clone());
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        let (x_min, x_max) = (0.0, self.nsteps as f64 * 0.1);
        let (y_min, y_max) = (0.0, 1.0);
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(20)
            .caption("BR0D - Gates", ("Arial", 20).into_font())
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Time (ms)")
            .draw()
            .unwrap();

        if gates.contains("m") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.msave.iter()).map(|(x, y)| (*x, *y)),
                    &RED,
                ))
                .unwrap();
        }

        if gates.contains("h") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.hsave.iter()).map(|(x, y)| (*x, *y)),
                    &GREEN,
                ))
                .unwrap();
        }

        if gates.contains("j") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.jsave.iter()).map(|(x, y)| (*x, *y)),
                    &BLUE,
                ))
                .unwrap();
        }

        if gates.contains("d") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.dsave.iter()).map(|(x, y)| (*x, *y)),
                    &YELLOW,
                ))
                .unwrap();
        }

        if gates.contains("f") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.fsave.iter()).map(|(x, y)| (*x, *y)),
                    &CYAN,
                ))
                .unwrap();
        }

        if gates.contains("x1") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.x1save.iter()).map(|(x, y)| (*x, *y)),
                    &MAGENTA,
                ))
                .unwrap();
        }
    }

    pub fn draw_current(&self, canvas: HtmlCanvasElement, current: &str) {
        self.clear(canvas.clone());
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        let (x_min, x_max) = (0.0, self.nsteps as f64 * 0.1);
        let (y_min, y_max) = (-5.0, 5.0);
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .margin(20)
            .caption("BR0D - Currents", ("Arial", 20).into_font())
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();

        chart
            .configure_mesh()
            .x_desc("Time (ms)")
            .y_desc("Current (uA/cm^2)")
            .draw()
            .unwrap();

        if current.contains("is") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.issave.iter()).map(|(x, y)| (*x, *y)),
                    &RED,
                ))
                .unwrap();
        }

        if current.contains("ik1") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.ik1save.iter()).map(|(x, y)| (*x, *y)),
                    &GREEN,
                ))
                .unwrap();
        }

        if current.contains("ix1") {
            chart
                .draw_series(LineSeries::new(
                    self.t.iter().zip(self.ix1save.iter()).map(|(x, y)| (*x, *y)),
                    &BLUE,
                ))
                .unwrap();
        }
    }

    pub fn calculate(&mut self) {
        let gna = 4.0;
        let gnac = 0.003;
        let ena = 50.0;
        let gs = 0.09;
        let cm = 1.0;

        let dt = 0.1;

        let period = 280.0;
        let mut istim;
        let stimmag = 26.4;

        let mut v = self.vsave[0];
        let mut m = self.msave[0];
        let mut h = self.hsave[0];
        let mut j = self.jsave[0];
        let mut d = self.dsave[0];
        let mut f = self.fsave[0];
        let mut x1 = self.x1save[0];
        let mut cai = self.caisave[0];

        let mut ina;
        let mut is;
        let mut ik1;
        let mut ix1;

        let mut c1: f64;
        let mut c2: f64;
        let mut c3: f64;
        let mut c4: f64;
        let mut c5: f64;
        let mut c6: f64;
        let mut c7: f64;

        let mut ax1: f64;
        let mut bx1: f64;
        let mut am: f64;
        let mut bm: f64;
        let mut ah: f64;
        let mut bh: f64;
        let mut aj: f64;
        let mut bj: f64;
        let mut ad: f64;
        let mut bd: f64;
        let mut af: f64;
        let mut bf: f64;

        let mut es: f64;
        let mut dcai: f64;
        let mut dv: f64;

        for i in 1..self.nsteps {
            c1 = 0.0005;
            c2 = 0.083;
            c3 = 50.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.057;
            c7 = 1.0;
            ax1 = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.0013;
            c2 = -0.06;
            c3 = 20.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.04;
            c7 = 1.0;
            bx1 = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.0;
            c2 = 0.0;
            c3 = 47.0;
            c4 = -1.0;
            c5 = 47.0;
            c6 = -0.1;
            c7 = -1.0;
            if abs(v + c3) < 0.00001 {
                am = 10.0;
            } else {
                am =
                    (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            }
            c1 = 40.0;
            c2 = -0.056;
            c3 = 72.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.0;
            c7 = 0.0;
            bm = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.126;
            c2 = -0.25;
            c3 = 77.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.0;
            c7 = 0.0;
            ah = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 1.7;
            c2 = 0.0;
            c3 = 22.5;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.082;
            c7 = 1.0;
            bh = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.055;
            c2 = -0.25;
            c3 = 78.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.2;
            c7 = 1.0;
            aj = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.3;
            c2 = 0.0;
            c3 = 32.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.1;
            c7 = 1.0;
            bj = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.095;
            c2 = -0.01;
            c3 = -5.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.072;
            c7 = 1.0;
            ad = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.07;
            c2 = -0.017;
            c3 = 44.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.05;
            c7 = 1.0;
            bd = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.012;
            c2 = -0.008;
            c3 = 28.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = 0.15;
            c7 = 1.0;
            af = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);
            c1 = 0.0065;
            c2 = -0.02;
            c3 = 30.0;
            c4 = 0.0;
            c5 = 0.0;
            c6 = -0.2;
            c7 = 1.0;
            bf = (c1 * f64::exp(c2 * (v + c3)) + c4 * (v + c5)) / (f64::exp(c6 * (v + c3)) + c7);

            m = (m + dt * am) / (1.0 + dt * (am + bm));
            h = (h + dt * ah) / (1.0 + dt * (ah + bh));
            j = (j + dt * aj) / (1.0 + dt * (aj + bj));
            d = (d + dt * ad) / (1.0 + dt * (ad + bd));
            f = (f + dt * af) / (1.0 + dt * (af + bf));
            x1 = (x1 + dt * ax1) / (1.0 + dt * (ax1 + bx1));

            es = -82.3 - 13.0287 * f64::log(cai, f64::exp(1.0));
            ik1 = 0.35
                * (4.0 * (f64::exp(0.04 * (v + 85.0)) - 1.0)
                    / (f64::exp(0.08 * (v + 53.0)) + f64::exp(0.04 * (v + 53.0)))
                    + 0.2 * (v + 23.0) / (1.0 - f64::exp(-0.04 * (v + 23.0))));
            ix1 = x1 * 0.8 * (f64::exp(0.04 * (v + 77.0)) - 1.0) / f64::exp(0.04 * (v + 35.0));
            ina = (gna * m * m * m * h * j + gnac) * (v - ena);
            is = gs * d * f * (v - es);

            dcai = -1.0e-7 * is + 0.07 * (1.0e-7 - cai);
            cai = cai + dt * dcai;

            istim = 0.0;
            if i % ((period / dt) as usize) < ((2.0 / dt) as usize) {
                istim = stimmag;
            }

            // dv=-(ik1+ix1+ina+is-istim)/cm;
            dv = -(ik1 + ix1 + ina + is - istim) / cm;
            v = v + dt * dv;

            self.vsave[i as usize] = v;
            self.msave[i as usize] = m;
            self.hsave[i as usize] = h;
            self.jsave[i as usize] = j;
            self.dsave[i as usize] = d;
            self.fsave[i as usize] = f;
            self.x1save[i as usize] = x1;
            self.caisave[i as usize] = cai;

            self.inasave[i as usize] = ina;
            self.issave[i as usize] = is;
            self.ik1save[i as usize] = ik1;
            self.ix1save[i as usize] = ix1;

        }
    }

    pub fn clear(&self, canvas: HtmlCanvasElement) {
        let backend = CanvasBackend::with_canvas_object(canvas).unwrap();
        let root = backend.into_drawing_area();
        root.fill(&WHITE).unwrap();
    }
}
