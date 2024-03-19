use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub struct FOX1D {
    x: Vec<f64>,
    v: Vec<f64>,
    ccai: Vec<f64>,
    ccasr: Vec<f64>,
    xf: Vec<f64>,
    xd: Vec<f64>,
    xm: Vec<f64>,
    xh: Vec<f64>,
    xj: Vec<f64>,
    xfca: Vec<f64>,
    xkr: Vec<f64>,
    xks: Vec<f64>,
    xto: Vec<f64>,
    yto: Vec<f64>,
    gna: f64,
    gk1_up: f64,
    gkr_up: f64,
    gks: f64,
    gkp: f64,
    gto: f64,
    gnab: f64,
    gcab: f64,
    pca_up: f64,
    pcak_up: f64,
    prel: f64,
    pleak: f64,
    xinakbar_up: f64,
    xicahalfbar: f64,
    xipcabar: f64,
    eta: f64,
    xksat: f64,
    xknaca_up: f64,
    xkmfca: f64,
    xkmnai_up: f64,
    xkmpca: f64,
    xkmup: f64,
    cmdntot_up: f64,
    csqntot_up: f64,
    xkmcmdn: f64,
    xkmcsqn: f64,
    vup: f64,
    vmyo_up: f64,
    cnai: f64,
    cki: f64,
    cnao: f64,
    cko: f64,
    ccao: f64,
    stim: f64,
    istimdur: usize,
    f: f64,
    rtof: f64,
    ena: f64,
    ek: f64,
    eks: f64,
    sigma_up: f64,
    taufca: f64,
    caidtconst: f64,
    nx: usize,
    dt: f64,
    dx: f64,
    diff_up: f64,
    outputevery: usize,
    i: usize,
    boundary: usize,
}

impl FOX1D {
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
impl FOX1D {
    // pub fn test(n: usize) -> FOX1D {
    //     FOX1D {}
    // }

    pub fn new(
        v: f64,
        ccai: f64,
        ccasr: f64,
        xf: f64,
        xd: f64,
        xm: f64,
        xh: f64,
        xj: f64,
        xfca: f64,
        xkr: f64,
        xks: f64,
        xto: f64,
        yto: f64,
        gna: f64,
        gk1: f64,
        gkr: f64,
        gks: f64,
        gkp: f64,
        gto: f64,
        gnab: f64,
        gcab: f64,
        pca: f64,
        pcak: f64,
        prel: f64,
        pleak: f64,
        xinakbar: f64,
        xicahalfbar: f64,
        xipcabar: f64,
        tt: f64,
        acap: f64,
        csc: f64,
        eta: f64,
        xksat: f64,
        xknaca: f64,
        xkmfca: f64,
        xkmk1: f64,
        xkmna: f64,
        xkmca: f64,
        xkmnai: f64,
        xkmko: f64,
        xkmpca: f64,
        xkmup: f64,
        cmdntot: f64,
        csqntot: f64,
        xkmcmdn: f64,
        xkmcsqn: f64,
        vup: f64,
        vmyo: f64,
        vsr: f64,
        cnai: f64,
        cki: f64,
        cnao: f64,
        cko: f64,
        ccao: f64,
        stim: f64,
        xstimdur: f64,
        nx: usize,
        dt: f64,
        dx: f64,
        diff: f64,
        outputevery: usize,
        boundary: usize,
    ) -> FOX1D {
        let f: f64 = 96.5;
        const R: f64 = 8.314;
        let rtof: f64 = R * tt / f;
        let ena = rtof * (f64::ln(cnao / cnai));
        let ek = rtof * (f64::ln(cko / cki));
        let eks = rtof * (f64::ln((cko + 0.01833 * cnao) / (cki + 0.01833 * cnai)));
        let sigma_up = 0.0365 * (f64::exp(cnao / 67.3) - 1.) / 7.;
        let taufca: f64 = 30.;
        let caidtconst = acap * csc * 0.5 / (f * vmyo);

        let gk1_up = gk1 * cko / (cko + xkmk1);
        let gkr_up = gkr * f64::sqrt(cko) * 0.5;
        let pca_up = pca / csc;
        let pcak_up = pcak / csc;
        let xinakbar_up = xinakbar * (cko / (cko + xkmko));
        let xknaca_up = xknaca / (xkmna * xkmna * xkmna + cnao * cnao * cnao) / (xkmca + ccao);
        let xkmnai_up = 1. + f64::powf(xkmnai / cnai, 1.5);
        let cmdntot_up = cmdntot * xkmcmdn;
        let csqntot_up = csqntot * xkmcsqn;
        let vmyo_up = vmyo / vsr;
        let istimdur = (xstimdur / dt).floor() as usize;
        let diff_up = diff * dt / (dx * dx);

        let x: Vec<f64> = (0..nx).map(|i| i as f64 * dx).collect();
        let v = vec![v; nx];
        let ccai = vec![ccai; nx];
        let ccasr = vec![ccasr; nx];
        let xf = vec![xf; nx];
        let xd = vec![xd; nx];
        let xm = vec![xm; nx];
        let xh = vec![xh; nx];
        let xj = vec![xj; nx];
        let xfca = vec![xfca; nx];
        let xkr = vec![xkr; nx];
        let xks = vec![xks; nx];
        let xto = vec![xto; nx];
        let yto = vec![yto; nx];

        FOX1D {
            x, v, ccai, ccasr, xf, xd, xm, xh, xj, xfca, xkr, xks, xto, yto,
            gna, gk1_up, gkr_up, gks, gkp, gto, gnab, gcab, pca_up, pcak_up, prel, pleak,
            xinakbar_up, xicahalfbar, xipcabar, eta, xksat, xknaca_up, xkmfca, xkmnai_up, xkmpca, xkmup,
            cmdntot_up, csqntot_up, xkmcmdn, xkmcsqn, vup, vmyo_up, cnai, cki, cnao, cko, ccao,
            stim, istimdur, f, rtof, ena, ek, eks, sigma_up, taufca, caidtconst,
            nx, dt, dx, diff_up, outputevery, i: 0, boundary
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
        let mut am: Vec<f64>;
        let mut bm: Vec<f64>;
        let mut ah: Vec<f64>;
        let mut bh: Vec<f64>;
        let mut aj: Vec<f64>;
        let mut bj: Vec<f64>;
        
        let mut taum1: Vec<f64>;
        let mut tauh1: Vec<f64>;
        let mut tauj1: Vec<f64>;
        let mut xminf1: Vec<f64>;
        let mut xhinf1: Vec<f64>;
        let mut xjinf1: Vec<f64>;

        let mut xina: Vec<f64>;

        let mut xik1: Vec<f64>;

        let mut xrinf1: Vec<f64>;
        let mut taukr1: Vec<f64>;
        let mut xikrcoeff: Vec<f64>;
        let mut xikr: Vec<f64>;

        let mut xsinf1: Vec<f64>;
        let mut tauks1: Vec<f64>;
        let mut xiks: Vec<f64>;

        let mut axto: Vec<f64>;
        let mut bxto: Vec<f64>;
        let mut ayto: Vec<f64>;
        let mut byto: Vec<f64>;
        let mut tauxto1: Vec<f64>;
        let mut tauyto1: Vec<f64>;
        let mut xtoinf1: Vec<f64>;
        let mut ytoinf1: Vec<f64>;
        let mut xito: Vec<f64>;

        let mut xikp: Vec<f64>;

        let mut xinak: Vec<f64>;

        let mut xinacat1: Vec<f64>;
        let mut xinacat2: Vec<f64>;
        let mut xinaca: Vec<f64>;

        let mut xipca: Vec<f64>;

        let mut ecat: Vec<f64>;
        let mut xicab: Vec<f64>;

        let mut xinab: Vec<f64>;

        let mut finf1: Vec<f64>;
        let mut tauf1: Vec<f64>;
        let mut dinf1: Vec<f64>;
        let mut taud1: Vec<f64>;
        let mut xicabart1: Vec<f64>;
        let mut xicabart2: Vec<f64>;
        let mut xicabar: Vec<f64>;
        let mut fcainf1: Vec<f64>;
        let mut xica: Vec<f64>;

        let mut xicakcoeff: Vec<f64>;
        let mut xicak: Vec<f64>;

        let mut gamma: Vec<f64>;
        let mut xjrelcoeff: Vec<f64>;
        let mut xjrel: Vec<f64>;
        let mut xjleak: Vec<f64>;
        let mut xjup: Vec<f64>;
        let mut bit: Vec<f64>;
        let mut dcaidt: Vec<f64>;
        let mut bsr: Vec<f64>;
        let mut dcasrdt: Vec<f64>;

        let mut xstim = vec![0.0; self.nx];

        let mut xlap: Vec<f64>;

        for ntime in self.i..self.i + self.outputevery {
            am = self.v.iter().map(|v| 0.32 * (v + 47.13) / (1.0 - f64::exp(-0.1 * (v + 47.13)))).collect();
            bm = self.v.iter().map(|v| 0.08 * f64::exp(-v / 11.0)).collect();
            ah = self.v.iter().map(|v| 0.135 * f64::exp(-(v + 80.0) / 6.8)).collect();
            bh = self.v.iter().map(|v| 7.5 / (1.0 + f64::exp(-0.1 * (v + 11.0)))).collect();
            aj = self.v.iter().map(|v| 0.175 * f64::exp(-(v + 100.0) / 23.0) / (1.0 + f64::exp(0.15 * (v + 79.0)))).collect();
            bj = self.v.iter().map(|v| 0.3 / (1.0 + f64::exp(-0.1 * (v + 32.0)))).collect();
            
            taum1 = am.iter().zip(bm.iter()).map(|(a, b)| 1.0 / (a + b)).collect();
            xminf1 = taum1.iter().zip(am.iter()).map(|(t, a)| t * a).collect();
            tauh1 = ah.iter().zip(bh.iter()).map(|(a, b)| 1.0 / (a + b)).collect();
            xhinf1 = tauh1.iter().zip(ah.iter()).map(|(t, a)| t * a).collect();
            tauj1 = aj.iter().zip(bj.iter()).map(|(a, b)| 1.0 / (a + b)).collect();
            xjinf1 = tauj1.iter().zip(aj.iter()).map(|(t, a)| t * a).collect();
            
            self.xm = self.xm.iter().zip(xminf1.iter()).zip(taum1.iter()).map(|((xm, x), t)| x + (xm - x) * f64::exp(-self.dt / t)).collect();
            self.xh = self.xh.iter().zip(xhinf1.iter()).zip(tauh1.iter()).map(|((xh, x), t)| x + (xh - x) * f64::exp(-self.dt / t)).collect();
            self.xj = self.xj.iter().zip(xjinf1.iter()).zip(tauj1.iter()).map(|((xj, x), t)| x + (xj - x) * f64::exp(-self.dt / t)).collect();
            
            xina = self.v.iter().zip(self.xm.iter()).zip(self.xh.iter()).zip(self.xj.iter()).map(|(((v, xm), xh), xj)| self.gna * xm * xm * xm * xh * xj * (v - self.ena)).collect();

            xik1 = self.v.iter().map(|v| self.gk1_up * (v - self.ek) / (2.0 + f64::exp(1.62 / self.rtof * (v - self.ek)))).collect();

            xrinf1 = self.v.iter().map(|v| 1.0 / (1.0 + f64::exp(-2.182 - 0.1819 * v))).collect();
            taukr1 = self.v.iter().map(|v| 43.0 + 1.0 / (f64::exp(-5.495 + 0.1691 * v) + f64::exp(-7.677 - 0.0128 * v))).collect();
            self.xkr = self.xkr.iter().zip(xrinf1.iter()).zip(taukr1.iter()).map(|((xkr, r), t)| r + (xkr - r) * f64::exp(-self.dt / t)).collect();
            xikrcoeff = self.v.iter().map(|v| self.gkr_up * (v - self.ek) / (1.0 + 2.5 * f64::exp(0.1 * (v + 28.0)))).collect();
            xikr = self.xkr.iter().zip(xikrcoeff.iter()).map(|(x, c)| x * c).collect();

            xsinf1 = self.v.iter().map(|v| 1.0 / (1.0 + f64::exp((v - 16.0) / -13.6))).collect();
            tauks1 = self.v.iter().map(|v| 1.0 / ((0.0000719 * (v - 10.0) / (1.0 - f64::exp(-0.148 * (v - 10.0))) + (0.000131 * (v - 10.0) / (f64::exp(0.0687 * (v - 10.0)) - 1.0))))).collect();
            self.xks = self.xks.iter().zip(xsinf1.iter()).zip(tauks1.iter()).map(|((xks, s), t)| s + (xks - s) * f64::exp(-self.dt / t)).collect();
            xiks = self.xks.iter().zip(self.v.iter()).map(|(x, v)| self.gks * x * x * (v - self.eks)).collect();

            axto = self.v.iter().map(|v| 0.04516 * f64::exp(0.03577 * v)).collect();
            bxto = self.v.iter().map(|v| 0.0989 * f64::exp(-0.06237 * v)).collect();
            ayto = self.v.iter().map(|v| 0.005415 * f64::exp((v + 33.5) / -5.0) / (1.0 + 0.051335 * f64::exp((v + 33.5) / -5.0))).collect();
            byto = self.v.iter().map(|v| 0.005415 * f64::exp((v + 33.5) / 5.0) / (1.0 + 0.051335 * f64::exp((v + 33.5) / 5.0))).collect();
            tauxto1 = axto.iter().zip(bxto.iter()).map(|(a, b)| 1.0 / (a + b)).collect();
            xtoinf1 = tauxto1.iter().zip(axto.iter()).map(|(t, a)| t * a).collect();
            tauyto1 = ayto.iter().zip(byto.iter()).map(|(a, b)| 1.0 / (a + b)).collect();
            ytoinf1 = tauyto1.iter().zip(ayto.iter()).map(|(t, a)| t * a).collect();

            self.xto = self.xto.iter().zip(xtoinf1.iter()).zip(tauxto1.iter()).map(|((xto, x), t)| x + (xto - x) * f64::exp(-self.dt / t)).collect();
            self.yto = self.yto.iter().zip(ytoinf1.iter()).zip(tauyto1.iter()).map(|((yto, y), t)| y + (yto - y) * f64::exp(-self.dt / t)).collect();
            xito = self.xto.iter().zip(self.yto.iter()).zip(self.v.iter()).map(|((x, y), v)| self.gto * x * y * (v - self.ek)).collect();

            xikp = self.v.iter().map(|v| self.gkp * (v - self.ek) / (1.0 + f64::exp((7.488 - v) / 5.98))).collect();

            xinak = self.v.iter().map(|v| self.xinakbar_up / (self.xkmnai_up) / (1.0 + 0.1245 * f64::exp(-0.1 * v / self.rtof) + self.sigma_up * f64::exp(-v / self.rtof))).collect();

            xinacat1 = self.v.iter().map(|v| self.xknaca_up / (1.0 + self.xksat * f64::exp(v * (self.eta - 1.0) / self.rtof)) * (f64::exp(v * self.eta / self.rtof) * f64::powi(self.cnai, 3) * self.ccao)).collect();
            xinacat2 = self.v.iter().map(|v| self.xknaca_up / (1.0 + self.xksat * f64::exp(v * (self.eta - 1.0) / self.rtof)) * (f64::exp(v * (self.eta - 1.0) / self.rtof) * f64::powi(self.cnao, 3))).collect();
            xinaca = xinacat1.iter().zip(xinacat2.iter()).zip(self.ccai.iter()).map(|((a, b), c)| a - b * c).collect();

            xipca = self.ccai.iter().map(|c| self.xipcabar * c / (self.xkmpca + c)).collect();

            ecat = self.ccai.iter().map(|c| 0.5 * self.rtof * f64::ln(self.ccao / c)).collect();
            xicab = ecat.iter().zip(self.v.iter()).map(|(e, v)| self.gcab * (v - e)).collect();

            xinab = self.v.iter().map(|v| self.gnab * (v - self.ena)).collect();

            finf1 = self.v.iter().map(|v| 1.0 / (1.0 + f64::exp((v + 12.5) / 5.0))).collect();
            tauf1 = self.v.iter().map(|v| 30.0 + 200.0 / (1.0 + f64::exp((v + 20.0) / 9.5))).collect();
            dinf1 = self.v.iter().map(|v| 1.0 / (1.0 + f64::exp((v + 10.0) / -6.24))).collect();
            taud1 = self.v.iter().map(|v| 1.0 / ((0.25 * f64::exp(-0.01 * v) / (1.0 + f64::exp(-0.07 * v)) + 0.07 * f64::exp(-0.05 * (v + 40.0)) / (1.0 + f64::exp(0.05 * (v + 40.0)))))).collect();

            xicabart1 = self.v.iter().map(|v| self.pca_up * 4.0 * v * self.f / self.rtof / (f64::exp(2.0 * v / self.rtof) - 1.0) * f64::exp(2.0 * v / self.rtof)).collect();
            xicabart2 = self.v.iter().map(|v| self.pca_up * 4.0 * v * self.f / self.rtof / (f64::exp(2.0 * v / self.rtof) - 1.0) * 0.341 * self.ccao).collect();
            xicabar = xicabart1.iter().zip(xicabart2.iter()).zip(self.ccai.iter()).map(|((a, b), c)| a * c - b).collect();

            self.xf = self.xf.iter().zip(finf1.iter()).zip(tauf1.iter()).map(|((xf, f), t)| f + (xf - f) * f64::exp(-self.dt / t)).collect();
            self.xd = self.xd.iter().zip(dinf1.iter()).zip(taud1.iter()).map(|((xd, d), t)| d + (xd - d) * f64::exp(-self.dt / t)).collect();

            fcainf1 = self.ccai.iter().map(|c| 1.0 / (1.0 + f64::powi(c / self.xkmfca, 3))).collect();
            self.xfca = self.xfca.iter().zip(fcainf1.iter()).map(|(xfca, f)| f + (xfca - f) * f64::exp(-self.dt / self.taufca)).collect();
            xica = xicabar.iter().zip(self.xf.iter()).zip(self.xd.iter()).zip(self.xfca.iter()).map(|(((a, b), c), d) | a * b * c * d).collect();

            xicakcoeff = self.v.iter().map(|v| self.pcak_up * (1000.0 * v * self.f / self.rtof) * (self.cki * f64::exp(v / self.rtof) - self.cko) / (f64::exp(v / self.rtof) - 1.0)).collect();
            xicak = xicakcoeff.iter().zip(self.xf.iter()).zip(self.xd.iter()).zip(self.xfca.iter()).zip(xicabar.iter()).map(|((((a, b), c), d), e)| a * b * c * d / (1.0 + e / self.xicahalfbar)).collect();

            gamma = self.ccasr.iter().map(|c| 1.0 / (1.0 + f64::powi(2000.0 / c, 3))).collect();
            xjrelcoeff = self.v.iter().map(|v| self.prel / (1.0 + 1.65 * f64::exp(v / 20.0))).collect();
            xjrel = xjrelcoeff.iter().zip(self.xf.iter()).zip(self.xd.iter()).zip(self.xfca.iter()).zip(gamma.iter()).zip(self.ccasr.iter()).zip(self.ccai.iter()).map(|((((((a, b), c), d), e), f), g)| a * b * c * d * (e * f - g)).collect();
            xjleak = self.ccasr.iter().zip(self.ccai.iter()).map(|(a, b)| self.pleak * (a - b)).collect();
            xjup = self.ccai.iter().map(|c| self.vup / (1.0 + f64::powi(self.xkmup / c, 2))).collect();
            bit = self.ccai.iter().map(|c| 1.0 / (1.0 + self.cmdntot_up / (f64::powi(self.xkmcmdn + c, 2)))).collect();
            dcaidt = bit.iter().zip(xjrel.iter()).zip(xjleak.iter()).zip(xjup.iter()).zip(xica.iter()).zip(xicab.iter()).zip(xipca.iter()).zip(xinaca.iter()).map(|(((((((a, b), c), d), e), f), g), h)| a * (b + c - d - self.caidtconst * (e + f + g - 2.0 * h))).collect();
            self.ccai = self.ccai.iter().zip(dcaidt.iter()).map(|(c, d)| c + self.dt * d).collect();

            bsr = self.ccasr.iter().map(|c| 1.0 / (1.0 + self.csqntot_up / (f64::powi(self.xkmcsqn + c, 2)))).collect();
            // dcasrdt=bsr.*(xjup-xjleak-xjrel)*vmyo/vsr;
            dcasrdt = bsr.iter().zip(xjup.iter()).zip(xjleak.iter()).zip(xjrel.iter()).map(|(((a, b), c), d)| a * (b - c - d) * self.vmyo_up).collect();
            self.ccasr = self.ccasr.iter().zip(dcasrdt.iter()).map(|(c, d)| c + self.dt * d).collect();

            if ntime < self.istimdur {
                for i in 0..5 {
                    xstim[i] = self.stim;
                }
            }

            xlap = self.matrix_vector_smart(self.v.clone(), self.nx, self.boundary);
            xlap = xlap.iter().map(|x| self.diff_up * x).collect();

            // v=v+xlap-dt*(xina+xik1+xito+xikp+xinab+xiks+xica+xinaca+xipca+xicab+xicak+xinak+xikr+xstim);
            self.v = self.v.iter().zip(xlap.iter()).zip(xina.iter()).zip(xik1.iter()).zip(xito.iter()).zip(xikp.iter()).zip(xinab.iter()).zip(xiks.iter()).zip(xica.iter()).zip(xinaca.iter()).zip(xipca.iter()).zip(xicab.iter()).zip(xicak.iter()).zip(xinak.iter()).zip(xikr.iter()).zip(xstim.iter()).map(
                |(((((((((((((((v, xlap), xina), xik1), xito), xikp), xinab), xiks), xica), xinaca), xipca), xicab), xicak), xinak), xikr), xstim)| v + xlap - self.dt * (xina + xik1 + xito + xikp + xinab + xiks + xica + xinaca + xipca + xicab + xicak + xinak + xikr + xstim)
            ).collect();
        }
        
        self.i += self.outputevery;
    }

    pub fn set_stimulus(&mut self, index: usize, v: f64) {
        self.v[index] = v;
    }

    pub fn set_boundary(&mut self, boundary: usize) {
        self.boundary = boundary;
    }
}
