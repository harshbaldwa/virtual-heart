import { BR0D, BR1D, FOX1D } from "virtual-heart";

const model = document.getElementById('model');
let optionGroup, option;
const models = require('./models.json');

// iterate through the models and add them to the dropdown
for (let key in models) {
    optionGroup = document.createElement('optgroup');
    optionGroup.label = key;
    model.appendChild(optionGroup);
    models[key].forEach((model) => {
        option = document.createElement('option');
        option.value = model["value"];
        option.text = model["name"];
        if (model["selected"] === true) {
            option.selected = true;
        }
        optionGroup.appendChild(option);
    });
}

const plots = document.getElementById('plots');
const controls = document.getElementById('controls');

model.addEventListener('change', function () {
    // clear plots
    plots.innerHTML = "";
    // clear controls
    controls.innerHTML = "";
    switch (model.value) {
        case "br0d":
            const gatesActivate = document.createElement('input');
            gatesActivate.type = "checkbox";
            gatesActivate.id = "gatesActivate";
            const gatesLabel = document.createElement('label');
            gatesLabel.htmlFor = "gatesActivate";
            gatesLabel.appendChild(document.createTextNode("Gates"));
            controls.appendChild(gatesActivate);
            controls.appendChild(gatesLabel);

            const currentActivate = document.createElement('input');
            currentActivate.type = "checkbox";
            currentActivate.id = "currentActivate";
            const currentLabel = document.createElement('label');
            currentLabel.htmlFor = "currentActivate";
            currentLabel.appendChild(document.createTextNode("Currents"));
            controls.appendChild(currentActivate);
            controls.appendChild(currentLabel);

            let glabels = ["m", "h", "j", "d", "f", "x1"];
            let ilabels = ["is", "ik1", "ix1"]

            const gatesDiv = document.createElement('div');
            gatesDiv.id = "gatesDiv";
            gatesDiv.style.display = "none";
            controls.appendChild(gatesDiv);
            for (let i = 0; i < glabels.length; i++) {
                const gate = document.createElement('input');
                gate.type = "checkbox";
                gate.id = glabels[i];
                gate.checked = i == 0;
                const gateLabel = document.createElement('label');
                gateLabel.htmlFor = glabels[i];
                gateLabel.appendChild(document.createTextNode(glabels[i]));
                gatesDiv.appendChild(gate);
                gatesDiv.appendChild(gateLabel);
            }

            const currentDiv = document.createElement('div');
            currentDiv.id = "currentDiv";
            currentDiv.style.display = "none";
            controls.appendChild(currentDiv);
            for (let i = 0; i < ilabels.length; i++) {
                const current = document.createElement('input');
                current.type = "checkbox";
                current.id = ilabels[i];
                current.checked = i == 0;
                const currentLabel = document.createElement('label');
                currentLabel.htmlFor = ilabels[i];
                currentLabel.appendChild(document.createTextNode(ilabels[i]));
                currentDiv.appendChild(current);
                currentDiv.appendChild(currentLabel);
            }

            let whichGates = glabels[0];
            let whichCurrents = ilabels[0];

            const voltage = document.createElement('canvas');
            voltage.id = "voltage";
            voltage.width = 800;
            voltage.height = 400;
            const gates = document.createElement('canvas');
            gates.id = "gates";
            gates.width = 800;
            gates.height = 400;
            const current = document.createElement('canvas');
            current.id = "current";
            current.width = 800;
            current.height = 400;

            plots.appendChild(voltage);

            let v_br0d = -83.5;
            let m_br0d = 0.0127;
            let h_br0d = 0.9824;
            let j_br0d = 0.9685;
            let d_br0d = 0.0033;
            let f_br0d = 0.9969;
            let x1_br0d = 0.1410;
            let cai_br0d = 1.8545e-7;
            let nsteps_br0d = 50000;
            const br0d = BR0D.new(v_br0d, m_br0d, h_br0d, j_br0d, d_br0d, f_br0d, x1_br0d, cai_br0d, nsteps_br0d);
            br0d.calculate();
            br0d.draw(voltage);

            gatesActivate.addEventListener('change', function () {
                if (this.checked) {
                    br0d.draw_gates(gates, whichGates);
                    gatesDiv.style.display = "block";
                    plots.appendChild(gates);
                } else {
                    gatesDiv.style.display = "none";
                    plots.removeChild(gates);
                }
            });

            currentActivate.addEventListener('change', function () {
                if (this.checked) {
                    br0d.draw_current(current, whichCurrents);
                    currentDiv.style.display = "block";
                    plots.appendChild(current);
                } else {
                    currentDiv.style.display = "none";
                    plots.removeChild(current);
                }
            });

            for (let i = 0; i < glabels.length; i++) {
                document.getElementById(glabels[i]).addEventListener('change', function () {
                    if (this.checked) {
                        whichGates += this.id;
                    } else {
                        whichGates = whichGates.replace(this.value, "");
                    }
                    br0d.draw_gates(gates, whichGates);
                });
            }

            for (let i = 0; i < ilabels.length; i++) {
                document.getElementById(ilabels[i]).addEventListener('change', function () {
                    if (this.checked) {
                        whichCurrents += this.id;
                    } else {
                        whichCurrents = whichCurrents.replace(this.value, "");
                    }
                    br0d.draw_current(current, whichCurrents);
                });
            }

            break;
        case "br1d":
            const voltage_br1d = document.createElement('canvas');
            voltage_br1d.id = "voltage";
            voltage_br1d.width = 800;
            voltage_br1d.height = 400;
            // margin of 50px
            voltage_br1d.style.margin = "50px";

            plots.appendChild(voltage_br1d);

            // add drop down to controls for periodic vs no-flux boundary conditions
            const boundary_control = document.createElement('select');
            boundary_control.id = "boundary";
            const noflux = document.createElement('option');
            noflux.value = "0";
            noflux.text = "No-flux";
            const periodic = document.createElement('option');
            periodic.value = "1";
            periodic.text = "Periodic";
            boundary_control.appendChild(noflux);
            boundary_control.appendChild(periodic);
            controls.appendChild(boundary_control);

            // listen for changes to the boundary condition
            boundary_control.addEventListener('change', function () {
                br1d.set_boundary(parseInt(this.value));
            });


            let v_br1d = -83.5;
            let m_br1d = 0.0127;
            let h_br1d = 0.9824;
            let j_br1d = 0.9685;
            let d_br1d = 0.0033;
            let f_br1d = 0.9969;
            let x1_br1d = 0.1410;
            let cai_br1d = 1.8545e-7;

            let gna_br1d = 4.0;
            let gnac_br1d = 0.003;
            let ena_br1d = 50.0;
            let gs_br1d = 0.09;
            let cm_br1d = 1.0;

            let dt_br1d = 0.02;
            // let dt_br1d = 0.03;
            let dx_br1d = 0.025;
            let diff_br1d = 0.001;
            // let diff_br1d = 0.00;
            let outputevery_br1d = 20;
            let nx_br1d = 1200;
            let period_br1d = 6100;
            let boundary_br1d = 0; // 0 = No-flux, 1 = Periodic
            const br1d = BR1D.new(v_br1d, m_br1d, h_br1d, j_br1d, d_br1d, f_br1d, x1_br1d, cai_br1d, gna_br1d, gnac_br1d, ena_br1d, gs_br1d, cm_br1d, dt_br1d, dx_br1d, diff_br1d, outputevery_br1d, nx_br1d, period_br1d, boundary_br1d);
            // run the animation
            setInterval(function () {
                br1d.tick();
                br1d.draw(voltage_br1d);
            }, 0);

            // make the canvas interactive
            voltage_br1d.addEventListener('click', function (e) {
                const rect = voltage_br1d.getBoundingClientRect();
                
                const x = e.clientX - rect.left;
                const y = e.clientY - rect.top;
                const i = Math.floor(x / (voltage_br1d.width / nx_br1d));
                const j = 100 - Math.floor(y / (voltage_br1d.height / 200));
                br1d.set_stimulus(i, j);
            });

            break;
        case "fox1d":
            const voltage_fox1d = document.createElement('canvas');
            voltage_fox1d.id = "voltage";
            voltage_fox1d.width = 800;
            voltage_fox1d.height = 400;
            // margin of 50px
            voltage_fox1d.style.margin = "50px";

            plots.appendChild(voltage_fox1d);

            // add drop down to controls for periodic vs no-flux boundary conditions
            const boundary_control_fox1d = document.createElement('select');
            boundary_control_fox1d.id = "boundary";
            const noflux_fox1d = document.createElement('option');
            noflux_fox1d.value = "0";
            noflux_fox1d.text = "No-flux";
            const periodic_fox1d = document.createElement('option');
            periodic_fox1d.value = "1";
            periodic_fox1d.text = "Periodic";
            boundary_control_fox1d.appendChild(noflux_fox1d);
            boundary_control_fox1d.appendChild(periodic_fox1d);
            controls.appendChild(boundary_control_fox1d);

            // listen for changes to the boundary condition
            boundary_control_fox1d.addEventListener('change', function () {
                fox1d.set_boundary(parseInt(this.value));
            });


            let v_f1 = -94.7;
            let ccai_f1 = 0.0472;
            let ccasr_f1 = 320.0;
            let xf_f1 = 1.0;
            let xd_f1 = 0.0;
            let xm_f1 = 0.0;
            let xh_f1 = 1.0;
            let xj_f1 = 1.0;
            let xfca_f1 = 1.0;
            let xkr_f1 = 0.0;
            let xks_f1 = 0.0;
            let xto_f1 = 0.0;
            let yto_f1 = 1.0;

            let gna_f1 = 12.8;
            let gk1_f1 = 2.8;
            let gkr_f1 = 0.0136;
            let gks_f1 = 0.0245;
            let gkp_f1 = 0.002216;
            let gto_f1 = 0.23815;
            let gnab_f1 = 0.0031;
            let gcab_f1 = 0.0003842;
            let pca_f1 = 0.0000226;
            let pcak_f1 = 5.79e-7;
            let prel_f1 = 6.0;
            let pleak_f1 = 0.000001;
            let xinakbar_f1 = 0.693;
            let xicahalfbar_f1 = -0.265;
            let xipcabar_f1 = 0.05;
            let tt_f1 = 310.0;
            let acap_f1 = 1.534e-4;
            let csc_f1 = 1.0;
            let eta_f1 = 0.35;
            let xksat_f1 = 0.2;
            let xknaca_f1 = 1500.0;
            let xkmfca_f1 = 0.18;
            let xkmk1_f1 = 13.0;
            let xkmna_f1 = 87.5;
            let xkmca_f1 = 1380.0;
            let xkmnai_f1 = 10.0;
            let xkmko_f1 = 1.5;
            let xkmpca_f1 = 0.05;
            let xkmup_f1 = 0.32;
            let cmdntot_f1 = 10.0;
            let csqntot_f1 = 10000.0;
            let xkmcmdn_f1 = 2.0;
            let xkmcsqn_f1 = 600.0;
            let vup_f1 = 0.1;
            let vmyo_f1 = 25.84e-6;
            let vsr_f1 = 2e-6;
            let cnai_f1 = 10.0;
            let cki_f1 = 149.4;
            let cnao_f1 = 138.0;
            let cko_f1 = 4.0;
            let ccao_f1 = 2000.0;
            let stim_f1 = -80.0;
            let xstimdur_f1 = 1.0;

            let nx_f1 = 1200;
            let dt_f1 = 0.02;
            let dx_f1 = 0.0125;
            let diff_f1 = 0.001;
            let outputevery_f1 = 20;
            let boundary_f1 = 0; // 0 = No-flux, 1 = Periodic
            
            const fox1d = FOX1D.new(v_f1, ccai_f1, ccasr_f1, xf_f1, xd_f1, xm_f1, xh_f1, xj_f1, xfca_f1, xkr_f1, xks_f1, xto_f1, yto_f1, gna_f1, gk1_f1, gkr_f1, gks_f1, gkp_f1, gto_f1, gnab_f1, gcab_f1, pca_f1, pcak_f1, prel_f1, pleak_f1, xinakbar_f1, xicahalfbar_f1, xipcabar_f1, tt_f1, acap_f1, csc_f1, eta_f1, xksat_f1, xknaca_f1, xkmfca_f1, xkmk1_f1, xkmna_f1, xkmca_f1, xkmnai_f1, xkmko_f1, xkmpca_f1, xkmup_f1, cmdntot_f1, csqntot_f1, xkmcmdn_f1, xkmcsqn_f1, vup_f1, vmyo_f1, vsr_f1, cnai_f1, cki_f1, cnao_f1, cko_f1, ccao_f1, stim_f1, xstimdur_f1, nx_f1, dt_f1, dx_f1, diff_f1, outputevery_f1, boundary_f1);

            // run the animation
            setInterval(function () {
                fox1d.tick();
                fox1d.draw(voltage_fox1d);
            }, 0);

            // make the canvas interactive
            voltage_fox1d.addEventListener('click', function (e) {
                const rect = voltage_fox1d.getBoundingClientRect();
                
                const x = e.clientX - rect.left;
                const y = e.clientY - rect.top;
                const i = Math.floor(x / (voltage_fox1d.width / nx_f1));
                const j = 100 - Math.floor(y / (voltage_fox1d.height / 200));
                fox1d.set_stimulus(i, j);
            });

            break;
        default:
            console.log("Model not found");
    }
});

model.value = "fox1d";
model.dispatchEvent(new Event('change'));