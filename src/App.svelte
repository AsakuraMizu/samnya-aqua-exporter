<script lang="ts">
  import { save } from '@tauri-apps/api/dialog';
  import { writeFile } from '@tauri-apps/api/fs';
  import { Body, Client, getClient } from '@tauri-apps/api/http';
  import { downloadDir } from '@tauri-apps/api/path';
  import { invoke } from '@tauri-apps/api/tauri';

  import {
    Accordion,
    AccordionItem,
    AppBar,
    AppShell,
    ProgressRadial,
    Toast,
    getToastStore,
    initializeStores,
  } from '@skeletonlabs/skeleton';

  import IconGitHub from '~icons/mdi/github';
  import IconCog from '~icons/mdi/cog';
  import IconAlert from '~icons/mdi/alert';
  import IconAccount from '~icons/mdi/account';
  import IconMagnify from '~icons/mdi/magnify';
  import IconDatabase from '~icons/mdi/database';
  import IconInformation from '~icons/mdi/information';

  initializeStores();

  let aimedb = 'aime.samnya.cn';
  let allnet = 'aime.samnya.cn';
  let path = 'ChuniServlet/2.10/A0/ChuniServlet';

  let accessCode: string | undefined;
  $: accessCodeValid = accessCode && /^\d{20}$/.test(accessCode);
  let aimeId: number | undefined;

  const toastStore = getToastStore();
  const getAimeId = async () => {
    if (!accessCodeValid) {
      toastStore.trigger({
        message: '无效卡号',
        background: 'variant-filled-error',
      });
      return;
    }
    const toastId = toastStore.trigger({
      message: '开始获取 AimeID...',
      hideDismiss: true,
      autohide: false,
    });
    try {
      aimeId = await invoke('get_aimeid', { host: aimedb, accessCode });
      if (aimeId === -1) {
        toastStore.close(toastId);
        toastStore.trigger({
          message: '未找到卡号对应用户, 请检查输入的卡号!',
          background: 'variant-filled-error',
        });
        aimeId = undefined;
      }
    } catch (e) {
      console.log(e);
      toastStore.close(toastId);
      toastStore.trigger({
        message: `获取失败！发生错误: ${e}`,
        background: 'variant-filled-error',
        hoverable: true,
      });
    }
  };

  let client: Client;
  const getUserXXApi = async (api: string, extra?: Record<string, string>) => {
    if (!aimeId) return;
    const url = `http://${allnet}/${path}/GetUser${api}Api`;
    const body = Body.json({ ...extra, userId: aimeId.toString() });
    console.log(url, body);
    client ??= await getClient();
    return (await client.post<any>(url, body)).data;
  };

  enum Status {
    Waiting,
    Fetching,
    Error,
    Fetched,
  }

  let activityList: any;
  let activityStatus = Status.Waiting;
  const getActivity = async () => {
    if (!aimeId || activityStatus === Status.Fetching) return;
    try {
      activityStatus = Status.Fetching;
      activityList = (
        await Promise.all([
          getUserXXApi('Activity', { kind: '1' }),
          getUserXXApi('Activity', { kind: '2' }),
          getUserXXApi('Activity', { kind: '4' }),
        ])
      ).flatMap((value) => value.userActivityList);
      if (!activityList) throw 'empty data';
      activityStatus = Status.Fetched;
    } catch (e: any) {
      activityList = e.toString();
      activityStatus = Status.Error;
    }
  };

  let characterList: any;
  let characterStatus = Status.Waiting;
  const getCharacter = async () => {
    if (!aimeId || characterStatus === Status.Fetching) return;
    try {
      characterStatus = Status.Fetching;
      characterList = (
        await getUserXXApi('Character', {
          nextIndex: '0',
          maxCount: '100000',
        })
      ).userCharacterList;
      if (!characterList) throw 'empty data';
      characterStatus = Status.Fetched;
    } catch (e: any) {
      characterList = e.toString();
      characterStatus = Status.Error;
    }
  };

  let chargeList: any;
  let chargeStatus = Status.Waiting;
  const getCharge = async () => {
    if (!aimeId || chargeStatus === Status.Fetching) return;
    try {
      chargeStatus = Status.Fetching;
      chargeList = (await getUserXXApi('Charge')).userChargeList;
      if (!chargeList) throw 'empty data';
      chargeStatus = Status.Fetched;
    } catch (e: any) {
      chargeList = e.toString();
      chargeStatus = Status.Error;
    }
  };

  let courseList: any;
  let courseStatus = Status.Waiting;
  const getCourse = async () => {
    if (!aimeId || courseStatus === Status.Fetching) return;
    try {
      courseStatus = Status.Fetching;
      courseList = (await getUserXXApi('Course')).userCourseList;
      if (!courseList) throw 'empty data';
      courseStatus = Status.Fetched;
    } catch (e: any) {
      courseList = e.toString();
      courseStatus = Status.Error;
    }
  };

  let data: any;
  let dataStatus = Status.Waiting;
  const getData = async () => {
    if (!aimeId || dataStatus === Status.Fetching) return;
    try {
      dataStatus = Status.Fetching;
      data = (await getUserXXApi('Data')).userData;
      if (!data) throw 'empty data';
      dataStatus = Status.Fetched;
    } catch (e: any) {
      data = e.toString();
      dataStatus = Status.Error;
    }
  };

  let duelList: any;
  let duelStatus = Status.Waiting;
  const getDuel = async () => {
    if (!aimeId || duelStatus === Status.Fetching) return;
    try {
      duelStatus = Status.Fetching;
      duelList = (
        await getUserXXApi('Duel', {
          duelId: '1',
          isAllDuel: 'true',
        })
      ).userDuelList;
      if (!duelList) throw 'empty data';
      duelStatus = Status.Fetched;
    } catch (e: any) {
      duelList = e.toString();
      duelStatus = Status.Error;
    }
  };

  let itemList: any;
  let itemStatus = Status.Waiting;
  const getItem = async () => {
    if (!aimeId || itemStatus === Status.Fetching) return;
    try {
      itemStatus = Status.Fetching;
      itemList = (
        await getUserXXApi('Item', {
          nextIndex: '0',
          maxCount: '100000',
        })
      ).userItemList;
      if (!itemList) throw 'empty data';
      itemStatus = Status.Fetched;
    } catch (e: any) {
      itemList = e.toString();
      itemStatus = Status.Error;
    }
  };

  let mapAreaList: any;
  let mapAreaStatus = Status.Waiting;
  const getMapArea = async () => {
    if (!aimeId || mapAreaStatus === Status.Fetching) return;
    try {
      mapAreaStatus = Status.Fetching;
      mapAreaList = (await getUserXXApi('MapArea')).userMapAreaList;
      if (!mapAreaList) throw 'empty data';
      mapAreaStatus = Status.Fetched;
    } catch (e: any) {
      mapAreaList = e.toString();
      mapAreaStatus = Status.Error;
    }
  };

  let musicDetailList: any;
  let musicDetailStatus = Status.Waiting;
  const getMusicDetail = async () => {
    if (!aimeId || musicDetailStatus === Status.Fetching) return;
    try {
      musicDetailStatus = Status.Fetching;
      musicDetailList = (
        await getUserXXApi('Music', {
          nextIndex: '0',
          maxCount: '100000',
        })
      ).userMusicList.flatMap((value: any) => value.userMusicDetailList);
      if (!musicDetailList) throw 'empty data';
      musicDetailStatus = Status.Fetched;
    } catch (e: any) {
      musicDetailList = e.toString();
      musicDetailStatus = Status.Error;
    }
  };

  let option: any;
  let optionStatus = Status.Waiting;
  const getOption = async () => {
    if (!aimeId || optionStatus === Status.Fetching) return;
    try {
      optionStatus = Status.Fetching;
      option = (await getUserXXApi('Option')).userGameOption;
      if (!option) throw 'empty data';
      optionStatus = Status.Fetched;
    } catch (e: any) {
      option = e.toString();
      optionStatus = Status.Error;
    }
  };

  const getAll = () => {
    getActivity();
    getCharacter();
    getCharge();
    getCourse();
    getData();
    getDuel();
    getItem();
    getMapArea();
    getMusicDetail();
    getOption();
  };

  const saveFile = async () => {
    const savePath = await save({
      filters: [{ name: 'JSON Document', extensions: ['json'] }],
      defaultPath: `${await downloadDir()}/${aimeId}.json`,
    });
    if (!savePath) return;
    writeFile(
      savePath,
      JSON.stringify(
        {
          gameId: 'SDHD',
          userData: data,
          userActivityList: activityList ?? [],
          userCharacterList: characterList ?? [],
          userChargeList: chargeList ?? [],
          userCourseList: courseList ?? [],
          userDuelList: duelList ?? [],
          userGameOption: option,
          userItemList: itemList ?? [],
          userMapList: mapAreaList ?? [],
          userMusicDetailList: musicDetailList ?? [],
          userPlaylogList: [],
        },
        (key, value) => {
          if (key.endsWith('Date') && typeof value === 'string') return value.replace(' ', 'T');
          return value;
        },
      ),
    );
  };
</script>

<Toast />
<AppShell slotSidebarLeft="bg-surface-500/5 p-4">
  <svelte:fragment slot="header">
    <!-- App Bar -->
    <AppBar>
      <svelte:fragment slot="lead">
        <strong class="text-xl">Samnya 服 (aqua) 数据导出工具</strong>
      </svelte:fragment>
      <svelte:fragment slot="trail">
        <a
          class="btn btn-sm variant-soft-surface"
          href="https://github.com/AsakuraMizu/samnya-aqua-exporter"
          target="_blank"
          rel="noreferrer"
        >
          <IconGitHub />
          <span>GitHub</span>
        </a>
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>

  <div class="container h-full mx-auto flex justify-center items-center">
    <Accordion class="w-full m-4">
      <AccordionItem>
        <svelte:fragment slot="lead"><IconCog /></svelte:fragment>
        <svelte:fragment slot="summary">服务器设置</svelte:fragment>
        <svelte:fragment slot="content">
          <aside class="alert variant-filled-warning">
            <IconAlert class="text-2xl" />
            <div class="alert-message">
              <h3 class="h3">警告</h3>
              <p>除非你知道自己在做什么，否则不要修改这些内容。</p>
            </div>
          </aside>

          <div class="w-full flex gap-4">
            <label class="label flex-1">
              <span>AimeDb</span>
              <input class="input" type="text" bind:value={aimedb} />
            </label>

            <label class="label flex-1">
              <span>ALL.Net</span>
              <input class="input" type="text" bind:value={allnet} />
            </label>

            <label class="label flex-1">
              <span>API 路径</span>
              <input class="input" type="text" bind:value={path} />
            </label>
          </div>
        </svelte:fragment>
      </AccordionItem>

      <AccordionItem open>
        <svelte:fragment slot="lead"><IconAccount /></svelte:fragment>
        <svelte:fragment slot="summary">账号</svelte:fragment>
        <svelte:fragment slot="content">
          <p>
            填入您的 AimeID. 如果您不知道您的 AimeID, 可以填入您的 Access Code (卡号) 并点击查询.
            <br />
            或者如果您在任何查分 bot 上绑定过账号, 您可以联系开发者获取 AimeID (例如 SiGNAL酱ᴮᴼᵀ 用户可以使用
            kt chuni.me 查询)
          </p>

          <div class="w-full flex flex-wrap gap-4">
            <label class="label flex-1 min-w-[200px]">
              <span>Access Code</span>
              <div
                class="input-group input-group-divider grid-cols-[1fr_auto]"
                class:input-error={!accessCodeValid}
              >
                <input
                  class="input"
                  type="text"
                  placeholder="Access Code"
                  bind:value={accessCode}
                />
                <button class="btn" on:click={getAimeId}><IconMagnify />查询</button>
              </div>
            </label>
            <label class="label flex-1 min-w-[200px]">
              <span>AimeID</span>
              <input class="input" type="number" placeholder="AimeID" bind:value={aimeId} />
            </label>
          </div>
        </svelte:fragment>
      </AccordionItem>

      <AccordionItem open>
        <svelte:fragment slot="lead"><IconDatabase /></svelte:fragment>
        <svelte:fragment slot="summary">获取数据</svelte:fragment>
        <svelte:fragment slot="content">
          <aside class="alert variant-filled-surface">
            <IconInformation class="text-2xl" />
            <div class="alert-message">
              <h3 class="h3">说明</h3>
              <p>由于 API 限制, Playlog (游玩记录) 数据暂时无法导出, 但不会影响游玩次数数据.</p>
            </div>
          </aside>

          <div class="w-full grid grid-cols-[1fr_1fr] gap-4">
            <button class="btn variant-filled-primary" disabled={!aimeId} on:click={getAll}>
              获取全部数据
            </button>
            <button
              class="btn variant-filled-secondary"
              disabled={dataStatus !== Status.Fetched}
              on:click={saveFile}
            >
              保存到文件
            </button>
            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {dataStatus ===
              Status.Waiting
                ? ''
                : dataStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : dataStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getData}
            >
              <header>Data (基础数据)</header>
              <section>
                {#if dataStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if dataStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if dataStatus === Status.Error}
                  <p>
                    发生错误: {data}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>
                    用户名: {data.userName}
                    <br />
                    ...(略)
                  </p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {optionStatus ===
              Status.Waiting
                ? ''
                : optionStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : optionStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getOption}
            >
              <header>Option (游戏选项)</header>
              <section>
                {#if optionStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if optionStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if optionStatus === Status.Error}
                  <p>
                    发生错误: {option}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取游戏选项</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {musicDetailStatus ===
              Status.Waiting
                ? ''
                : musicDetailStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : musicDetailStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getMusicDetail}
            >
              <header>MusicDetail (成绩)</header>
              <section>
                {#if musicDetailStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if musicDetailStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if musicDetailStatus === Status.Error}
                  <p>
                    发生错误: {musicDetailList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {musicDetailList.length} 条成绩</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {characterStatus ===
              Status.Waiting
                ? ''
                : characterStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : characterStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getCharacter}
            >
              <header>Character (角色)</header>
              <section>
                {#if characterStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if characterStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if characterStatus === Status.Error}
                  <p>
                    发生错误: {characterList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {characterList.length} 条角色数据</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {courseStatus ===
              Status.Waiting
                ? ''
                : courseStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : courseStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getCourse}
            >
              <header>Course (段位)</header>
              <section>
                {#if courseStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if courseStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if courseStatus === Status.Error}
                  <p>
                    发生错误: {courseList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {courseList.length} 条段位数据</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {mapAreaStatus ===
              Status.Waiting
                ? ''
                : mapAreaStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : mapAreaStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getMapArea}
            >
              <header>MapArea (地图进度)</header>
              <section>
                {#if mapAreaStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if mapAreaStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if mapAreaStatus === Status.Error}
                  <p>
                    发生错误: {mapAreaList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {mapAreaList.length} 条地图进度数据</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {duelStatus ===
              Status.Waiting
                ? ''
                : duelStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : duelStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getDuel}
            >
              <header>Duel (チュウニズムデュエル)</header>
              <section>
                {#if duelStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if duelStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if duelStatus === Status.Error}
                  <p>
                    发生错误: {duelList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {duelList.length} 条 Duel 数据</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {itemStatus ===
              Status.Waiting
                ? ''
                : itemStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : itemStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getItem}
            >
              <header>Item (物品)</header>
              <section>
                {#if itemStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if itemStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if itemStatus === Status.Error}
                  <p>
                    发生错误: {itemList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {itemList.length} 条物品数据</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {activityStatus ===
              Status.Waiting
                ? ''
                : activityStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : activityStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getActivity}
            >
              <header>Activity (不知道什么东西)</header>
              <section>
                {#if activityStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if activityStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if activityStatus === Status.Error}
                  <p>
                    发生错误: {activityList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {activityList.length} 条数据</p>
                {/if}
              </section>
              <div />
            </button>

            <button
              class="card card-hover p-4 h-32 flex flex-col items-center justify-between {chargeStatus ===
              Status.Waiting
                ? ''
                : chargeStatus === Status.Fetching
                  ? 'variant-filled-surface'
                  : chargeStatus === Status.Error
                    ? 'variant-filled-error'
                    : 'variant-filled-success'} {!aimeId && 'cursor-not-allowed opacity-75'}"
              on:click={getCharge}
            >
              <header>Charge (还是不知道什么东西)</header>
              <section>
                {#if chargeStatus === Status.Waiting}
                  <p>等待获取...</p>
                {:else if chargeStatus === Status.Fetching}
                  <ProgressRadial width="w-14" />
                {:else if chargeStatus === Status.Error}
                  <p>
                    发生错误: {chargeList}
                    <br />
                    点击重试
                  </p>
                {:else}
                  <p>已获取 {chargeList.length} 条数据</p>
                {/if}
              </section>
              <div />
            </button>
          </div>
        </svelte:fragment>
      </AccordionItem>
    </Accordion>
  </div>
</AppShell>
