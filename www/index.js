import {
    init,
} from './common.js';
import {
    makeVariable,
    makeApplication,
    makeLambda,
} from './lambda.js';
import {
    drawTerm,
    initSvgRoot
} from './draw.js';
import * as wasm from "lambda-wasm";

init(() => {
    const svgTarget = document.getElementById('tree');
    initSvgRoot(svgTarget);
    const term = makeApplication(
        makeLambda('x', makeVariable('x')),
        makeLambda('y', makeApplication(makeVariable('f'), makeVariable('y'))),
    );
    drawTerm(term, svgTarget);
});
