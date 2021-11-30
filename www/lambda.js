export function throwNonLambda() {
    throw new Error('Expected a variable, application or lambda');
}

export function makeVariable(name) {
    return { varname: name };
}

export function isVariable(term) {
    return 'varname' in term;
}

export function makeApplication(func, argument) {
    return { function: func, argument: argument };
}

export function isApplication(term) {
    return 'function' in term && 'argument' in term;
}

export function makeLambda(parameter, body) {
    return { parameter: parameter, body: body };
}

export function isLambda(term) {
    return 'parameter' in term && 'body' in term;
}

export function stringify(term) {
    if (isVariable(term)) {
        return term.varname;
    }

    if (isApplication(term)) {
        let func = stringify(term.function);
        if (isLambda(term.function)) {
            func = '(' + func + ')';
        } 

        let arg = stringify(term.argument);
        if (isLambda(term.argument) || isApplication(term.argument)) {
            arg = '(' + arg + ')';
        } 

        return func + ' ' + arg;
    }

    if (isLambda(term)) {
        return '\\' + term.parameter + '. ' + stringify(term.body);
    } 

    throwNonLambda();
}
