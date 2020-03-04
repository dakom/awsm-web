export class MainMenuToggle extends CustomEvent<{label: string}> {
    constructor(detail: {label:string}) {
        super("main-menu-toggle", { detail });
    }
}