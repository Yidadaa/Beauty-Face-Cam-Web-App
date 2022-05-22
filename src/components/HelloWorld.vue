<script setup lang="ts">
import { ref } from 'vue'
import init, { greet, printObject, quickSort, Point, quickSortCmp } from '../../core/pkg'

defineProps<{ msg: string }>()

function quickSortJs(arr: number[], left: number, right: number) {
  if (left < right) {
    const pivot = arr[Math.floor((left + right) / 2)];
    let i = left;
    let j = right;
    while (i <= j) {
      while (arr[i] < pivot) {
        i++;
      }
      while (arr[j] > pivot) {
        j--;
      }
      if (i <= j) {
        const temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        i++;
        j--;
      }
    }
    quickSortJs(arr, left, j);
    quickSortJs(arr, i, right);
  }
}

// @ts-ignore
window.compare = (a: number, b: number) => {
  return a < b ? -1 : a > b ? 1 : 0;
}

const message = ref<string[]>([])

function measure(name: string, fn: () => void) {
  const start = performance.now()
  fn()
  const end = performance.now()
  message.value.push(`${name}: ${(end - start).toFixed(2)}ms`)
}

function randomArray(count: number) {
  const arr = []
  for (let i = 0; i < count; i++) {
    arr.push(Math.floor(Math.random() * 1000000))
  }
  return arr
}

const count = ref(0)
const increase = () => {
  count.value++
  init().then(() => {
    greet()

    let p = new Point(1, 2)
    printObject(p)
    console.log(p.get_distance_to_zero())

    const arr = randomArray(1000000)
    const typedArr = new Float64Array(arr)
    const typedArr2 = new Float64Array(arr)

    measure('quickSort', () => {
      quickSort(typedArr)
    })

    measure('quickSortJs', () => {
      quickSortJs(arr, 0, arr.length - 1)
    })

    measure('quickSortCmp', () => {
      quickSortCmp(typedArr2)
    })
  })
}

</script>

<template>
  <h1>{{ msg }} {{ count }}</h1>
  <button @click="increase">+1</button>
  <div v-for="(val, index) in message">{{ val }}</div>
</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
