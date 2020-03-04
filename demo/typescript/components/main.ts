import {LitElement, customElement, property, css} from "lit-element";
import {nothing, html} from "lit-html";
import main_css from "@styles/main.css";

@customElement("app-main")
export class Main extends LitElement {
    static styles = main_css;

    @property( { type : Number}  ) len = 0;

    render() {
        return html`${nothing}`;
    }
}