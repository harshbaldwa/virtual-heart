import { BR0D } from "virtual-heart";

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
            
            const v = -83.5;
            const m = 0.0127;
            const h = 0.9824;
            const j = 0.9685;
            const d = 0.0033;
            const f = 0.9969;
            const x1 = 0.1410;
            const cai = 1.8545e-7;
            const nsteps = 50000;
            const br0d = BR0D.new(v, m, h, j, d, f, x1, cai, nsteps);
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
        default:
            console.log("Model not found");
    }
});
