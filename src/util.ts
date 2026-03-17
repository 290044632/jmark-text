export class Util {
    static elementHasClass(el: HTMLElement, classNames: Array<string>) {
        const targetClassName = el.className;
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
