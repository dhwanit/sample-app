<script lang="ts">
  import { invoke } from "@tauri-apps/api";
	import { listen, type UnlistenFn } from "@tauri-apps/api/event";
	import { onDestroy, onMount } from "svelte";

  const onLogin = () => invoke('login', { apiKey: "Ad4iUn8Aa3i7fwA3q5sJkyUj4o8fjkf", username: "AB0101", password: "R@ndOmP@$$wOrd123!", mfaEncodedSecret: "5BJK5V35KRK5V3OWHJK5ZHIHSNSVTDU3"});
  const onAddMargin = () => invoke('add_margin', { marginAmount: 300000 });

  const requestTokenListener = async (e: any) => {
    console.log('request-token: ', e?.data);
  }
  const encodedTokenListener = async (e: any) => {
    console.log('encoded-token: ', e?.data);
  }
  const addMarginCompleteListener = async (e: any) => {
    console.log('add-margin-complete');
  }

  let unlistenRequestToken: UnlistenFn;
  let unlistenEncodedToken: UnlistenFn;
  let unlistenAddMarginComplete: UnlistenFn;

  onMount(async () => {
    unlistenRequestToken = await listen<string>('request-token', requestTokenListener);
    unlistenEncodedToken = await listen<string>('encoded-token', encodedTokenListener);
    unlistenAddMarginComplete = await listen<string>('add-margin-complete', addMarginCompleteListener);
  });

  onDestroy(() => {
    unlistenRequestToken();
    unlistenEncodedToken();
    unlistenAddMarginComplete();
  });
</script>

<div class="max-w-3xl mx-auto">
  <div class="p-2 justify-center items-center flex">
    <button class="btn btn-md variant-outline-secondary" on:click={onLogin}>Login</button>
  </div>
  <div class="p-2 justify-center items-center flex h1">
    <button class="btn btn-md variant-outline-secondary" on:click={onAddMargin}>Add Margin</button>
  </div>
</div>
