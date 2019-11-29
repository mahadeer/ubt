<script>
  import { afterUpdate } from "svelte";

  export let name;
  export let title;
  export let menu;
  export let description;
  export let syntax;
  export let properties;
  export let examples;
  export let requires;
  export let related;

  let isNew = false;
  if (name == "") {
    isNew = true;
  }

  $: actionURL = "reference/" + name;

  let onFormSubmit = _ => {
    if (name !== "") {
      fetch(actionURL, {
        method: "post",
        body: JSON.stringify({
          name: name,
          title: title,
          menu: menu,
          description: description,
          syntax: syntax,
          properties: properties,
          examples: examples,
          requires: requires,
          related: related
        })
      })
        .then(res => {
          if (!res.ok) {
            throw Error(res.statusText);
          }
          return true;
        })
        .catch(console.error);
    }
  };

  let addProperty = () => {
    properties = [
      ...properties,
      {
        key: "newKey",
        description: "",
        possibleValues: "",
        default: "",
        required: ""
      }
    ];
  };

  let removeProperty = idx => {
    properties = properties.filter((_, i) => i != idx);
  };

  let addExample = () => {
    examples = [
      ...examples,
      {
        description: "new example",
        content: "",
        note: ""
      }
    ];
  };

  let removeExample = idx => {
    examples = examples.filter((_, i) => i != idx);
  };

  let addRequire = () => {
    requires = [
      ...requires,
      {
        text: "new require",
        route: "/"
      }
    ];
  };

  let removeRequire = idx => {
    requires = requires.filter((_, i) => i != idx);
  };

  let addRelated = () => {
    related = [
      ...related,
      {
        text: "new related",
        route: "/"
      }
    ];
  };

  let removeRelated = idx => {
    related = related.filter((_, i) => i != idx);
  };
</script>

<style>
  .uk-table thead {
    background: #4caf50;
  }
  .uk-table thead th {
    color: white;
  }
  [contenteditable]:focus {
    border: 2px solid #333;
  }
</style>

{#if isNew}
  <h3>New Topic</h3>
{:else}
  <h3>Edit Topic</h3>
{/if}
<div class="uk-container uk-container-small uk-margin">
  <div class="uk-margin" uk-grid="first-column:;">
    <div class="uk-width-1-3@s">
      <label class="uk-form-label" for="form-stacked-text">
        Name
        <sup>*</sup>
      </label>
      <div class="uk-form-controls">
        <div class="uk-inline uk-width-1-1">
          {#if isNew}
            <input
              class="uk-input"
              type="text"
              required
              placeholder="Name"
              bind:value={name} />
          {:else}
            <input
              class="uk-input"
              type="text"
              required
              disabled
              placeholder="Name"
              bind:value={name} />
          {/if}

        </div>
      </div>
    </div>
    <div class="uk-width-1-3@s">
      <label class="uk-form-label" for="form-stacked-text">
        Title
        <sup>*</sup>
      </label>
      <div class="uk-form-controls">
        <div class="uk-inline uk-width-1-1">
          <input
            class="uk-input"
            type="text"
            required
            placeholder="Title"
            bind:value={title} />
        </div>
      </div>
    </div>
    <div class="uk-width-1-3@s">
      <label class="uk-form-label" for="form-stacked-text">
        Menu
        <sup>*</sup>
      </label>
      <div class="uk-form-controls">
        <div class="uk-inline uk-width-1-1">
          <input
            class="uk-input"
            type="text"
            required
            placeholder="Menu"
            bind:value={menu} />
        </div>
      </div>
    </div>
  </div>
  <div class="uk-margin">
    <label class="uk-form-label" for="form-stacked-text">Synatx</label>
    <div class="uk-form-controls">
      <textarea
        class="uk-textarea custom-text-area"
        maxlength="500"
        rows="3"
        bind:value={syntax}
        style="resize: none;" />
    </div>
  </div>
  <div class="uk-margin">
    <label class="uk-form-label" for="form-stacked-text">Description</label>
    <div class="uk-form-controls">
      <textarea
        class="uk-textarea custom-text-area"
        maxlength="500"
        rows="5"
        bind:value={description}
        style="resize: none;" />
    </div>
  </div>
  <div class="uk-margin">
    <h3 class="uk-heading-line" for="form-stacked-text">Properties</h3>
    <div class="uk-form-controls">
      <button
        class="uk-button-small uk-button-default uk-align-right"
        on:click={addProperty}>
        Add Property
      </button>
      <table class="uk-table">
        <thead>
          <tr>
            <th>key</th>
            <th>description</th>
            <th>possibleValues</th>
            <th>default</th>
            <th>required</th>
            <th>actions</th>
          </tr>
        </thead>
        <tbody>
          {#if properties.length == 0}
            <tr>
              <td colspan="6">No properties</td>
            </tr>
          {/if}
          {#each properties as prop, idx}
            <tr>
              <td contenteditable="true" bind:textContent={prop.key} />
              <td contenteditable="true" bind:textContent={prop.description} />
              <td
                contenteditable="true"
                bind:textContent={prop.possibleValues} />
              <td contenteditable="true" bind:textContent={prop.default} />
              <td contenteditable="true" bind:textContent={prop.required} />
              <td>
                <div class="uk-button-group">
                  <button
                    class="uk-button uk-button-small uk-button-danger"
                    on:click={() => removeProperty(idx)}>
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
  <div class="uk-margin">
    <h3 class="uk-heading-line" for="form-stacked-text">Examples</h3>
    <div class="uk-form-controls">
      <button
        class="uk-button-small uk-button-default uk-align-right"
        on:click={addExample}>
        Add Example
      </button>
      <table class="uk-table">
        <thead>
          <tr>
            <th>description</th>
            <th>content</th>
            <th>note</th>
            <th>actions</th>
          </tr>
        </thead>
        <tbody>
          {#if examples.length == 0}
            <tr>
              <td colspan="4">No examples added</td>
            </tr>
          {/if}
          {#each examples as example, idx}
            <tr>
              <td
                contenteditable="true"
                bind:textContent={example.description} />
              <td contenteditable="true" bind:textContent={example.content} />
              <td contenteditable="true" bind:textContent={example.note} />
              <td>
                <div class="uk-button-group">
                  <button
                    class="uk-button uk-button-small uk-button-danger"
                    on:click={() => removeExample(idx)}>
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
  <div class="uk-margin">
    <h3 class="uk-heading-line" for="form-stacked-text">Requires</h3>
    <div class="uk-form-controls">
      <button
        class="uk-button-small uk-button-default uk-align-right"
        on:click={addRequire}>
        Add Require
      </button>
      <table class="uk-table">
        <thead>
          <tr>
            <th>text</th>
            <th>route</th>
            <th width="10%">actions</th>
          </tr>
        </thead>
        <tbody>
          {#if requires.length == 0}
            <tr>
              <td colspan="3">No required properties</td>
            </tr>
          {/if}
          {#each requires as requ, idx}
            <tr>
              <td contenteditable="true" bind:textContent={requ.text} />
              <td contenteditable="true" bind:textContent={requ.route} />
              <td>
                <div class="uk-button-group">
                  <button
                    class="uk-button uk-button-small uk-button-danger"
                    on:click={() => removeRequire(idx)}>
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
  <div class="uk-margin">
    <h3 class="uk-heading-line" for="form-stacked-text">Related</h3>
    <div class="uk-form-controls">
      <button
        class="uk-button-small uk-button-default uk-align-right"
        on:click={addRelated}>
        Add Related
      </button>
      <table class="uk-table">
        <thead>
          <tr>
            <th>text</th>
            <th>route</th>
            <th width="10%">actions</th>
          </tr>
        </thead>
        <tbody>
          {#if related.length == 0}
            <tr>
              <td colspan="3">No related properties</td>
            </tr>
          {/if}
          {#each related as r, idx}
            <tr>
              <td contenteditable="true" bind:textContent={r.text} />
              <td contenteditable="true" bind:textContent={r.route} />
              <td>
                <div class="uk-button-group">
                  <button
                    class="uk-button uk-button-small uk-button-danger"
                    on:click={() => removeRelated(idx)}>
                    Delete
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  </div>
  <input
    type="button"
    class="uk-align-right uk-button uk-button-primary"
    value="Update"
    on:click={onFormSubmit} />
</div>
