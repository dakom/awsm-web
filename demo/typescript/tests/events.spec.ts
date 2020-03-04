if (typeof CustomEvent === "undefined") {
  (global as any)["CustomEvent"] = 
    class CustomEvent<T> {
        type:string;
        detail:T;

        constructor(name:string, props:{detail: T} ) {
            this.type = name;
            this.detail = props ? props.detail : null;
        }
    }
}

import {
    AddTodo,
    RemoveTodo,
    ToggleTodo,
    ChangeTodo,
    ToggleAllTodos,
    ClearCompleted,
    Reposition
} from "@events/events";
import { DropSide } from "@events/types/types";


const wasm = require("../../_static/wasm/core/pkg/my_core");

describe("check event types", () => {
    test("AddTodo", () => {
        const event = new AddTodo({label: "hello"});
        expect(wasm.check_rust_event_AddTodo(event)).toEqual(JSON.stringify(event.detail));
    });

    test("RemoveTodo", () => {
        const event = new RemoveTodo({id: [1,0]});
        expect(wasm.check_rust_event_RemoveTodo(event)).toEqual(JSON.stringify(event.detail));
    });

    test("ToggleTodo", () => {
        const event = new ToggleTodo({id: [1,0], complete: true});
        expect(wasm.check_rust_event_ToggleTodo(event)).toEqual(JSON.stringify(event.detail));
    });
    test("ChangeTodo", () => {
        const event = new ChangeTodo({id: [1,0], label: "hello"});
        expect(wasm.check_rust_event_ChangeTodo(event)).toEqual(JSON.stringify(event.detail));
    });
    test("ToggleAllTodos", () => {
        const event = new ToggleAllTodos(true);
        expect(wasm.check_rust_event_ToggleAllTodos(event)).toEqual(JSON.stringify(event.detail));
    });
    test("ClearCompleted", () => {
        const event = new ClearCompleted();
        expect(wasm.check_rust_event_ClearCompleted(event)).toEqual(JSON.stringify(event.detail));
    });
    test("Reposition", () => {
        const event = new Reposition({
            src: [1,0],
            dest: [2,0],
            side: DropSide.Before
        });

        expect(wasm.check_rust_event_Reposition(event)).toEqual(JSON.stringify(event.detail));
    });
});