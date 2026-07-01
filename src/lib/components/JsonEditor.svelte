<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { EditorView, basicSetup } from "codemirror";
  import { EditorState, Compartment } from "@codemirror/state";
  import { json } from "@codemirror/lang-json";
  import { oneDark } from "@codemirror/theme-one-dark";
  import { collection } from "../store";
  import { resolve as resolveTheme } from "../theme";

  export let value = "";
  export let readonly = false;
  /** Called after the document changes (for autosave wiring). */
  export let onChange: (() => void) | undefined = undefined;

  let el: HTMLDivElement;
  let view: EditorView | undefined;
  const themeC = new Compartment();
  $: dark = resolveTheme($collection.settings.theme) === "dark";

  const baseTheme = EditorView.theme({
    "&": { height: "100%", fontSize: "12px", background: "transparent" },
    ".cm-scroller": { overflow: "auto", fontFamily: "ui-monospace, monospace" },
    ".cm-gutters": { background: "transparent", border: "none" },
    "&.cm-focused": { outline: "none" },
  });

  /** Pretty-print valid JSON; leave invalid text untouched. */
  function tryFormat() {
    if (readonly) return;
    try {
      const p = JSON.stringify(JSON.parse(value), null, 2);
      if (p !== value) value = p;
    } catch { /* invalid JSON — leave as typed */ }
  }

  onMount(() => {
    view = new EditorView({
      doc: value,
      parent: el,
      extensions: [
        basicSetup,
        json(),
        baseTheme,
        EditorState.readOnly.of(readonly),
        EditorView.editable.of(!readonly),
        themeC.of(dark ? oneDark : []),
        EditorView.updateListener.of((u) => {
          if (u.docChanged && !readonly) {
            value = u.state.doc.toString();
            onChange?.();
          }
        }),
        EditorView.domEventHandlers({ blur: () => tryFormat() }),
      ],
    });
  });
  onDestroy(() => view?.destroy());

  // Push external value changes (auto-format, programmatic) into the editor.
  $: if (view && value !== view.state.doc.toString()) {
    view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: value } });
  }
  // Swap theme when light/dark changes.
  $: if (view) view.dispatch({ effects: themeC.reconfigure(dark ? oneDark : []) });
</script>

<div class="cm" bind:this={el}></div>
