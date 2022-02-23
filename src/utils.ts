const units = [
  ['year', 31536000],
  ['month', 2592000],
  ['day', 86400],
  ['hour', 3600],
  ['minute', 60],
  ['second', 1]
]

const duration = (timeAgoInSeconds: number) => {
  console.log(timeAgoInSeconds)
  for (const [name, seconds] of units) {
    const interval = Math.floor(timeAgoInSeconds / (seconds as number))
    if (interval >= 1) {
      return {
        interval: interval,
        unit: name
      }
    }
  }
}

const secondsAgo = (date: Date) => {
  return Math.floor((new Date().getTime() - date.getTime()) / 1000)
}

export function timeAgo (date: Date) {
  console.log(date)
  if (secondsAgo(date) === 0) {
    return 'just now'
  } else if (secondsAgo(date) < 0) {
    return 'in the future'
  }

  const { interval, unit } = duration(secondsAgo(date))
  const suffix = interval === 1 ? '' : 's'
  return `${interval} ${unit}${suffix} ago`
}
