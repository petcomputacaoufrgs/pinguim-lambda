export function throwNonLambda() {
    throw 'Expected a variable, application or lambda'
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
        let func = stringifyTerm(term.function);
        if (isLambda(term.function)) {
            func = '(' + func + ')';
        } 

        let arg = stringifyTerm(term.argument);
        if (isLambda(term.argument) || isApplication(term.argument)) {
            arg = '(' + arg + ')';
        } 

        return func + ' ' + arg;
    }

    if (isLambda(term)) {
        return '\\' + term.paramter + '. ' + stringifyTerm(term.body);
    } 

    throwNonLambda();
}
