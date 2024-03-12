import { BR0D, BR1D } from "virtual-heart";

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

            plots.appendChild(voltage_br1d);

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
            let dx_br1d = 0.025;
            let diff_br1d = 0.001;
            let outputevery_br1d = 20;
            let nx_br1d = 1200;
            let boundary_br1d = 1; // 0 = No-flux, 1 = Periodic
            const br1d = BR1D.new(v_br1d, m_br1d, h_br1d, j_br1d, d_br1d, f_br1d, x1_br1d, cai_br1d, gna_br1d, gnac_br1d, ena_br1d, gs_br1d, cm_br1d, dt_br1d, dx_br1d, diff_br1d, outputevery_br1d, nx_br1d, boundary_br1d);
            // run the animation
            setInterval(function () {
                br1d.tick();
                br1d.draw(voltage_br1d);
            }, 0);

            // const start = performance.now();
            // for (let i = 0; i < 500; i++) {
            //     br1d.tick();
            //     br1d.draw(voltage_br1d);
            // }
            // const end = performance.now();
            // console.log("Average time to run 500 steps: " + (end - start)/500 + "ms");
            break;
        default:
            console.log("Model not found");
    }
});
