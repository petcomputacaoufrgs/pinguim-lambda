import {
    init,
} from './common.js';
import {
    makeVariable,
    makeApplication,
    makeLambda,
    stringify,
} from './lambda.js';
import {
    drawTerm,
    initSvgRoot
} from './draw.js';
import * as wasm from "lambda-wasm";

const actualBtn = document.getElementById('upload_button');
const fileChosen = document.getElementById('file-chosen');
actualBtn.addEventListener('change', function(){
    fileChosen.textContent = this.files[0].name
});


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
                                    makeVariable('x')
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
