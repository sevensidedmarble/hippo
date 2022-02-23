import { writable } from 'svelte/store'

interface User {
  email: string
  id: string
}

export const user = writable<User | undefined>(null)
