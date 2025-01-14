<script lang="ts">
  import { getLangFromUrl, useTranslations } from "../i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
</script>

{#if window.innerWidth < 600}
  <div class="overflow-x-auto">
    <table class="table">
      <!-- head -->
      <thead>
        <tr>
          <th>Name</th>
          <th>E-mail</th>
          <th>Admin</th>
          <th> </th>
        </tr>
      </thead>
      <tbody>
        <!-- row 1 -->
        <tr>
          <td>Guilherme</td>
          <td>admin@gmail.com</td>
          <td>Yes</td>
          <th>
            <button class="btn btn-xs btn-info">Edit</button><button
              class="btn btn-xs btn-error">Delete</button
            >
          </th>
          <td></td>
        </tr>
      </tbody>
    </table>
  </div>
  <style>
    div {
      width: 100%;
    }
    th {
      color: var(--text-color);
      text-align: center;
    }
    td {
      color: var(--text-color);
      font-size: 0.7rem;
    }
    button {
      align-self: center;
    }
  </style>
{:else}
  <div class="overflow-x-auto">
    <table class="table">
      <!-- head -->
      <thead>
        <tr>
          <th> </th>
          <th>Name</th>
          <th>E-mail</th>
          <th>Admin</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <!-- row 1 -->
        <tr>
          <td>1</td>
          <td>Guilherme</td>
          <td>admin@gmail.com</td>
          <td>Yes</td>
          <th class="flex flex-col space-y-2">
            <button class="btn btn-xs btn-info">Edit</button>
            <button class="btn btn-xs btn-error">Delete</button>
          </th>
        </tr>
      </tbody>
    </table>
  </div>
  <style>
    div {
      width: 100%;
    }
    th {
      color: var(--text-color);
    }
    td {
      color: var(--text-color);
    }
  </style>
{/if}
