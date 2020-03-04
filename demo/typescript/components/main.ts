import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import main_css from "@styles/main.css";
import {MainMenuToggle} from "@events/events";
import dat from "dat.gui";

@customElement("app-main")
export class Main extends LitElement {
    static styles = main_css;
    private gui:dat.GUI;

    @property( { type : Number}  ) len = 0;

    firstUpdated(_changedProperties) {
        //const on_toggle_all = (evt:any) => this.dispatchEvent(new MainMenuToggle(evt.target.checked));
        this.gui = new dat.GUI();
    }

    render() {
        return html`${nothing}`;
    }
}