<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        />

        <q-toolbar-title>
          MeS Editor
        </q-toolbar-title>
        <div>ver.{{ p.version }}</div>

      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      show-if-above
      bordered
    >
      <q-list>
        <q-item-label header>lab</q-item-label>
        <EssentialLinkTo
          v-for="link in labList"
          :key="link.title"
          v-bind="link"
        >

        </EssentialLinkTo>
      </q-list>
      <q-list>
        <q-item-label header>MeS</q-item-label>
        <EssentialLink
          v-for="link in essentialLinks"
          :key="link.title"
          v-bind="link"
        />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import EssentialLink from 'components/EssentialLink.vue';
import EssentialLinkTo from 'components/EssentialLinkTo.vue';

import p from '../../package.json';

const labList = [
  {
    title: 'Meditor',
    caption: 'MeSのテキストエディタ',
    icon: 'code',
    link: '/'
  },
  {
    title: '字幕実験',
    caption: '音声ファイルにMeSから字幕をつける実験',
    icon: 'chat',
    link: '/sub'
  }
]

const linksList = [
  {
    title: 'Docs',
    caption: 'github.com/iranika/MeS/wiki',
    icon: 'school',
    link: 'https://github.com/iranika/MeS/wiki'
  },
  {
    title: 'Github',
    caption: 'github.com/iranika/MeS',
    icon: 'code',
    link: 'https://github.com/iranika/MeS/'
  },
  {
    title: 'Twitter',
    caption: '@happy_packet',
    icon: 'chat',
    link: 'https://twitter.com/happy_packet'
  },
  
];

export default defineComponent({
  name: 'MainLayout',

  components: {
    EssentialLink,
    EssentialLinkTo
  },

  setup () {
    const leftDrawerOpen = ref(false)

    return {
      p,
      essentialLinks: linksList,
      leftDrawerOpen,
      toggleLeftDrawer () {
        leftDrawerOpen.value = !leftDrawerOpen.value
      },
      labList: labList,
    }
  }
});
</script>
