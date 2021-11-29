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
    const term = makeLambda(
        'f',
        makeLambda(
            'x',
            makeApplication(
                makeApplication(
                    makeLambda(
                        'f',
                        makeLambda(
                            'x',
                            makeApplication(
                                makeVariable('f'),
                                makeApplication(
                                    makeVariable('f'),
                                    makeVariable('ajsdhashdhasdhhasdh')
                                ),
                            ),
                        ),
                    ),
                    makeVariable('f'),
                ),
                makeApplication(
                    makeApplication(
                        makeLambda(
                            'f',
                            makeLambda(
                                'x',
                                makeApplication(
                                    makeVariable('f'),
                                    makeApplication(
                                        makeVariable('f'),
                                        makeApplication(
                                            makeVariable('f'),
                                            makeVariable('x')
                                        ),
                                    ),
                                ),
                            ),
                        ),
                        makeVariable('f'),
                    ),
                    makeVariable('x'),
                ),
            ),
        ),
    );
    drawTerm(term, svgTarget);
});
