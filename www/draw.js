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

export function drawTerm(term, targetSvg, configChanges) {
    drawTermWith(term, targetSvg, new Config(configChanges));
}

function drawTermWith(term, targetSvg, config) {
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

function svgWidth(node, targetSvg) {
    let inserted = false;
    let element = node;
    while (!inserted && element.parentNode != null) {
        if (element.parentNode == targetSvg) {
            inserted = true;
        } else {
            element = element.parentNode;
        }
    }
    if (!inserted) {
        targetSvg.appendChild(element);
    }
    let width = node.getBBox().width;
    if (!inserted) {
        targetSvg.removeChild(element);
    }
    if (width == 0) {
        throw new Error("Element with zero width");
    }
    return width;
}

function drawVariable(term, targetSvg, config) {
    let textNode = createText(
        term.varname,
        'lambda-drawing-variable',
        config.variable.node
    );
    let bgNode = createBg(textNode, targetSvg, config.variable.node);
    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, config);
    targetSvg.appendChild(wrapper);
    return config.symmetricPos(svgWidth(bgNode, targetSvg));
}

function drawLambda(term, targetSvg, config) {
    let textNode = createText(
        'λ' + term.parameter,
        'lambda-drawing-lambda',
        config.lambda.node
    );

    let bgNode = createBg(textNode, targetSvg, config.lambda.node);
    let childConfig = config.clone();
    
    childConfig.setMinCenter(
        config.symmetricCenter(svgWidth(bgNode, targetSvg))
    );
    childConfig.top += config.lambda.line.height + config.lambda.node.height;
    let childPos = drawTermWith(term.body, targetSvg, childConfig);

    let newConfig = config.clone();
    newConfig.minCenter = childPos.center;
    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, newConfig);
    targetSvg.appendChild(wrapper);

    let position = newConfig.symmetricPos(svgWidth(bgNode, targetSvg));
    position.left = Math.min(position.left, childPos.left);
    position.right = Math.max(position.right, childPos.right);

    let line = createLine(
        position.center,
        newConfig.top + config.lambda.node.radius,
        0,
        newConfig.lambda.line,
    );
    targetSvg.appendChild(line);

    return position;
}

function drawApplication(term, targetSvg, config) {
    let isRedex = isLambda(term.function);
    let configName, nodeClass;

    if (isRedex) {
        configName = 'redexApp';
        nodeClass = 'lambda-drawing-redex-app';
    } else {
        configName = 'nonRedexApp';
        nodeClass = 'lambda-drawing-non-redex-app';
    }

    let textNode = createText(
        '@',
        'lambda-drawing-lambda ' + nodeClass,
        config[configName].node
    );

    let bgNode = createBg(textNode, targetSvg, config[configName].node);
    let leftChildConfig = config.clone();
    leftChildConfig.setMinCenter(
        config.symmetricCenter(svgWidth(bgNode, targetSvg))
        - config.minLeafDistance / 2
    );
    leftChildConfig.top += (
        config[configName].line.height
        + config[configName].node.height
    );
    let leftChildPos = drawTermWith(term.function, targetSvg, leftChildConfig);

    let rightChildConfig = config.clone();
    rightChildConfig.left = (
        leftChildPos.right + config.minLeafDistance
    );
    rightChildConfig.setMinCenter(rightChildConfig.left);
    rightChildConfig.top += (
        config[configName].line.height
        + config[configName].node.height
    );
    let rightChildPos = drawTermWith(
        term.argument,
        targetSvg,
        rightChildConfig
    );

    let newConfig = config.clone();
    newConfig.setMinCenter(
        (leftChildPos.center + rightChildPos.center) / 2
    );

    let wrapper = createNodeWrapper(textNode, bgNode, targetSvg, newConfig);
    targetSvg.appendChild(wrapper);
    let center = newConfig.symmetricCenter(svgWidth(bgNode, targetSvg));

    let position = new Position(leftChildPos.left, center, rightChildPos.right);
    
    let leftLine = createLine(
        position.center,
        newConfig.top + config.lambda.node.radius,
        leftChildPos.center - position.center,
        newConfig[configName].line,
    );
    targetSvg.appendChild(leftLine);

    let rightLine = createLine(
        position.center,
        newConfig.top + config.lambda.node.radius,
        rightChildPos.center - position.center,
        newConfig[configName].line,
    );
    targetSvg.appendChild(rightLine);

    return position;
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

function createBg(textNode, targetSvg, nodeConfig) {
    let ellipseNode = createSvgElem('ellipse');
    ellipseNode.setAttribute('rx', Math.max(
        nodeConfig.radius,
        nodeConfig.textPadding/2 + svgWidth(textNode, targetSvg),
    ));
    ellipseNode.setAttribute('ry', nodeConfig.radius);
    ellipseNode.setAttribute('fill', nodeConfig.fillColor);
    ellipseNode.setAttribute('stroke', nodeConfig.strokeColor);
    ellipseNode.setAttribute('cx', 0);
    ellipseNode.setAttribute('cy', 0);
    return ellipseNode;
}

function createLine(left, top, dx, config) {
    let lineNode = createSvgElem('line');
    lineNode.setAttribute('x1', 0);
    lineNode.setAttribute('y1', 0);
    lineNode.setAttribute('x2', dx);
    lineNode.setAttribute('y2', config.height);
    lineNode.setAttribute('stroke-width', config.width);
    lineNode.setAttribute('stroke', config.color);
    let gNode = createSvgElem('g');
    gNode.appendChild(lineNode);
    gNode.setAttribute(
        'transform',
        'translate(' + left + ',' + top + ')'
    );
    return gNode;
}

function createNodeWrapper(textNode, bgNode, targetSvg, config) {
    let gNode = createSvgElem('g');
    gNode.appendChild(bgNode);
    gNode.appendChild(textNode);

    let outerGNode = createSvgElem('g');
    outerGNode.appendChild(gNode);
    let center = config.symmetricCenter(svgWidth(bgNode, targetSvg));
    outerGNode.setAttribute(
        'transform',
        'translate(' + center + ',' + config.top + ')'
    );

    console.log(center);

    return outerGNode;
}

class Position {
    constructor(left, center, right) {
        this.left = left;
        this.center = center;
        this.right = right;
    }
}

class Config {
    constructor(changes) {
        changes = changes || {};
        
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

    setMinCenter(newMinCenter) {
        this.minCenter = Math.max(this.minCenter, newMinCenter);
    }

    actualLeft(width) {
        let centerOffset = width / 2;
        return Math.max(this.left, this.minCenter - centerOffset);
    }

    actualRight(width) {
        let left = this.actualLeft(width);
        return left + width;
    }

    symmetricCenter(width) {
        let left = this.actualLeft(width);
        let centerOffset = width / 2;
        return left + centerOffset;
    }

    symmetricPos(width) {
        return new Position(
            this.actualLeft(width),
            this.symmetricCenter(width),
            this.actualRight(width),
        );
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

    get height() {
        return this.radius * 2;
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
            strokeColor: '#e0e0ff',
            textColor: '#ff0000',
        }));
    }

    static defaultForLambda(changes) {
        return new this(assignConfig(Object.create(changes), changes, {
            textColor: '#008000',
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
    for (let key in fallback) {
        assignConfigKey(dest, key, source, fallback[key]);
    }
    return dest;
}
