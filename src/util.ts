export class Util {
    static elementHasClass(el: HTMLElement, classNames: Array<string>) {
        const targetClassName = el.className;
        console.log('elementHasClass', targetClassName, classNames);
        if (targetClassName) {
            return targetClassName.split(' ').filter(className => {
                return className.trim() != '';
            }).find(className => {
                return classNames.includes(className);
            });
        }
        return false;
    }
} 
