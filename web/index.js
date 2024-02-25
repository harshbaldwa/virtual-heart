import { BR0D } from "virtual-heart";

const canvas = document.getElementById('canvas');

const v = -83.5;
const m = 0.0127;
const h = 0.9824;
const j = 0.9685;
const d = 0.0033;
const f = 0.9969;
const x1 = 0.1410;
const cai = 1.8545e-7;

const br0d = BR0D.new(v, m, h, j, d, f, x1, cai);

br0d.calculate();
br0d.draw(canvas);
