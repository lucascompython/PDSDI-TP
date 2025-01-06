<script>
  import Layout from "@layouts/Layout.astro";
  import NavBar from "@components/NavBar.astro";
  import DropdownTemperature from "@components/DropdownTemperature.svelte";
  import DropdownColor from "@components/DropdownColor.svelte";
  import Footer from "@components/Footer.astro";

  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  const t = useTranslations(getLangFromUrl(Astro.url));
</script>

{#if window.innerWidth < 600}
  <Layout title="Clothe Match">
    <main>
      <NavBar />
      <div class="split-screen">
        <div class="left">
          <DropdownTemperature windowLocation={Astro.url} />
          <div class="margin-top">
            <DropdownColor windowLocation={Astro.url} />
          </div>
        </div>
      </div>
      <Footer />
    </main>

    <style>
      main {
        color: white;
        display: flex;
        flex-direction: column;
        flex: 1;
        background-color: #d9d9d9;
      }
      .split-screen {
        flex: 1;
        display: flex;
        width: 100%;

        justify-content: center;
      }
      :global(.dark) main {
        background-color: #0d1117;
      }
      .margin-top {
        margin-top: 1rem;
      }
    </style>
  </Layout>
{:else}
  <Layout title="Clothe Match">
    <main>
      <NavBar />
      <div class="split-screen">
        <div class="left">
          <div class="margin-left">
            <DropdownTemperature windowLocation={Astro.url} />
            <div class="margin-top">
              <DropdownColor windowLocation={Astro.url} />
            </div>
          </div>
          <div class="line-horizontal"></div>
        </div>
        <div class="right">Right content</div>
        <div class="line-horizontal"></div>
      </div>
      <Footer />
    </main>

    <style>
      main {
        color: white;
        display: flex;
        flex-direction: column;
        flex: 1;
        background-color: #d9d9d9;
      }
      .split-screen {
        flex: 1;
        display: flex;
        width: 100%;
        position: relative;
      }
      .left {
        flex-basis: 50%;
        display: flex;
        flex-direction: column;
      }
      :global(.dark) main {
        background-color: #0d1117;
      }
      .right {
        border-left: 3px solid #b3b1b1;
        flex-basis: 50%;
      }
      .margin-left {
        margin-left: 2rem;
      }

      .margin-top {
        margin-top: 1rem;
      }
    </style>
  </Layout>
{/if}
