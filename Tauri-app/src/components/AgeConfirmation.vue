
<template>
  <v-dialog v-model="dialog" max-width="400">
    <v-card>
      <v-card-title class="headline">Confirm Age</v-card-title>
      <v-card-text>
        Are you over 18 years old?
      </v-card-text>
      <v-card-actions>
        <v-btn color="primary" @click="ageConfirm">Yes</v-btn>
        <v-btn color="secondary" @click="ageCancel">No</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
