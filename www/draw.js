import {
    isVariable,
    isApplication,
    isLambda,
    throwNonLambda
} from './lambda.js';

export function initSvgRoot(targetSvg) {
    document.body.addEventListener('touchstart', dragStart, false);
    document.body.addEventListener('touchend', dragEnd, false);
    document.body.addEventListener('touchmove', drag, false);

    document.body.addEventListener('mousedown', dragStart, false);
    document.body.addEventListener('mouseup', dragEnd, false);
    document.body.addEventListener('mousemove', drag, false);
    targetSvg.addEventListener('resize', resize, false);

    let current = { x: 0, y: 0 };
    let dragging = false;

    resize();

    function resize() {
        targetSvg.removeAttribute('width');
        targetSvg.removeAttribute('height');

        targetSvg.setAttribute(
            'width',
            targetSvg.width.baseVal.value
        );
        targetSvg.setAttribute(
            'height',
            targetSvg.height.baseVal.value
        );
    }

    function move(offset) {
        let attribute = targetSvg.getAttribute('viewBox');
        let pieces;
        if (attribute == null || attribute == undefined) {
            pieces = [
                0,
                0,
                targetSvg.width.baseVal.value,
                targetSvg.height.baseVal.value,
            ];
        } else {
            pieces = attribute.split(' ').map(str => parseInt(str.trim()));
        }
        pieces[0] -= offset.x;
        pieces[1] -= offset.y;
        targetSvg.setAttribute('viewBox', pieces.join(' '));
    }

    function dragStart(evt) {
        if (evt.target == targetSvg) {
            if (evt.type == 'touchstart') {
                current.x = evt.touches[0].clientX;
                current.y = evt.touches[0].clientY;
            } else {
                current.x = evt.clientX;
                current.y = evt.clientY;
            }

            dragging = true;
        }
    }

    function dragEnd(evt) {
        dragging = false;
    }

    function drag(evt) {
        if (dragging) {
            evt.preventDefault();

            let offset = { x: 0, y: 0 };

            if (evt.type == 'touchstart') {
                offset.x = evt.touches[0].clientX - current.x;
                offset.y = evt.touches[0].clientY - current.y;
            } else {
                offset.x = evt.clientX - current.x;
                offset.y = evt.clientY - current.y;
            }

            move(offset);
            current.x += offset.x;
            current.y += offset.y;
        }
    }
}

function createSvgElem(name) {
    return document.createElementNS('http://www.w3.org/2000/svg', name)
}

export function clearSvg(targetSvg, config) {
    targetSvg.replaceChildren();
}

export function drawTerm(term, targetSvg, config) {
    if (isVariable(term)) {
        return drawVariable(term, targetSvg, config);
    }

    if (isApplication(term)) {
        return drawApplication(term, targetSvg, config);
    }

    if (isLambda(term)) {
        return drawLambda(term, targetSvg, config);
    }

    throwNonLambda();
}

function svgWidth(node) {
    return node.width.baseVal.value;
}

function drawVariable(term, targetSvg, config) {
    let textNode = createText(
        term.varname,
        'lambda-drawing-variable',
        config.variable.node
    );
    let bgNode = createBg(textNode, config.variable.node);
    let wrapper = createNodeWrapper(textNode, bgNode, config);
    return config.symmetricPos(svgWidth(bgNode));
}

function createText(content, nodeClass, nodeConfig) {
    let textNode = createSvgElem('text');
    textNode.setAttribute('fill', nodeConfig.textColor);
    textNode.setAttribute('text-anchor', 'middle');
    textNode.setAttribute(
        'transform',
        'translate(0,' + nodeConfig.textPadding + ')'
    );
    textNode.textContent = content;
    textNode.setAttribute('class', 'lambda-drawing ' + nodeClass);
    return textNode;
}

function createBg(textNode, nodeConfig) {
    let ellipseNode = createSvgElem('ellipse');
    ellipseNode.setAttribute('rx', Math.max(
        nodeConfig.radius,
        nodeConfig.textPadding + svgWidth(textNode),
    ));
    ellipseNode.setAttribute('ry', nodeConfig.backgroundColor);
    ellipseNode.setAttribute('fill', nodeConfig.fillColor);
    ellipseNode.setAttribute('stroke', nodeConfig.strokeColor);
    ellipseNode.setAttribute('cx', 0);
    ellipseNode.setAttribute('cy', 0);
    return ellipseNode;
}

function createNodeWrapper(textNode, bgNode, config) {
    let gNode = createSvgElem('g');
    gNode.appendChild(bgNode);
    gNode.appendChild(textNode);

    let outerGNode = createSvgElem('g');
    outerGNode.appendChild(gNode);
    let actualLeft = config.actualLeft(svgWidth(bgNode));
    outerGNode.setAttribute(
        'transform',
        'translate(' + actualLeft + ',' + config.top + ')'
    );

    return outerGNode;
}

class Position {
    constructor(left, center, right) {
        this.left = left;
        this.center = center;
        this.right = right;
    }

    static fromSubPos(lefts, center, rights) {
        let left = center;
        let right = center;

        for (let pos of lefts) {
            left = Math.min(left, pos.left);
        }

        for (let pos of rights) {
            right = Math.max(right, pos.right);
        }

        return new this(left, center, right);
    }
}

class Config {
    constructor(changes) {
        assignConfig(this, changes, {
            top: 40,
            left: 40,
            minCenter: 0,
            minLeafDistance: 20,
            onclick: term => console.log(term),
        });
        this.variable = {
            node: NodeConfig.defaultForVar(
                'variable' in changes ? changes.variable.node : {}
            ),
        };
        this.nonRedexApp = {
            node: NodeConfig.defaultForNonRedexApp(
                'nonRedexApp' in changes ? changes.nonRedexApp.node : {}
            ),
            line: new LineConfig(
                'nonRedexApp' in changes ? changes.nonRedexApp.line : {}
            ),
        };
        this.redexApp = {
            node: NodeConfig.defaultForRedexApp(
                'redexApp' in changes ? changes.redexApp.node : {}
            ),
            line: new LineConfig(
                'redexApp' in changes ? changes.redexApp.line : {}
            ),
        };
        this.lambda = {
            node: NodeConfig.defaultForLambda(
                'lambda' in changes ? changes.lambda.node : {}
            ),
            line: new LineConfig(
                'lambda' in changes ? changes.lambda.line : {}
            ),
        };
    }

    actualLeft(width) {
        let centerOffset = Math.trunc(width / 2);
        return Math.max(this.left, this.minCenter - centerOffset);
    }

    symmetricPos(width) {
        let left = this.actualLeft(width);
        let centerOffset = Math.trunc(width / 2);
        return new Position(left, left + centerOffset, left + width);
    }

    clone() {
        return new this.constructor(this);
    }
}

class NodeConfig {
    constructor(changes) {
        assignConfig(this, changes, {
            radius: 14,
            textPadding: 5,
            fillColor: '#ffffff',
            strokeColor: '#ffffff',
            textColor: '#000000',
        });
    }

    static defaultForVar(changes) {
        return new this(changes);
    }

    static defaultForNonRedexApp(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            textColor: '#ff6000',
        }));
    }

    static defaultForRedexApp(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            fillColor: '#e0e0ff',
            strokeColor: '#ff0000',
            textColor: '#ff0000',
        }));
    }

    static defaultForLambda(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            textColor: '#00ff00',
        }));
    }
}

class LineConfig {
    constructor(changes) {
        assignConfig(this, changes, {
            width: 2,
            color: '#0000ff',
            height: 50,
        });
    }
}

function assignConfigKey(dest, key, source, fallback) {
    if (key in source) {
        dest[key] = source[key];
    } else {
        dest[key] = fallback;
    }
    return dest;
}

function assignConfig(dest, source, fallback) {
    source = source || null;
    for (key in source) {
        assignConfigKey(dest, key, source, fallback[key]);
    }
    return dest;
}
