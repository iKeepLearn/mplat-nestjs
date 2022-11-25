import * as url from "url"
import * as loadsh from 'lodash'
import * as CryptoJS from 'crypto-js';
import * as dayjs from 'dayjs'
import 'dayjs/locale/zh-cn'
import * as utc from 'dayjs/plugin/utc'
import * as timezone from 'dayjs/plugin/timezone'

dayjs.locale('zh-cn')
dayjs.extend(utc)
dayjs.extend(timezone)
dayjs.tz.setDefault("Asia/Shanghai")

const cryptoSalt = 'superMan';

export function formatNumber(n) {
  n = n.toString()
  return n[1] ? n : '0' + n
}

export function formatDateTime(date, str, hasTime) {

  let datestr = dayjs(date).tz("Asia/Shanghai").format(`YYYY${str}MM${str}DD`)
  if (hasTime) {
    datestr = dayjs(date).tz("Asia/Shanghai").format(`YYYY${str}MM${str}DD HH:mm:ss`)
  }
  return datestr

}

export function getDateTime(time) {
  return dayjs.tz(time).toDate()
}

export function getYear(date) {
  const year = new Date(date).getFullYear()
  return year
}

export function getMonth(date) {
  const month = new Date(date).getMonth() + 1
  return formatNumber(month)
}

export function isAfterTime(time1, time2) {
  const timeOne = dayjs.tz(time1)
  const timeTwo = dayjs.tz(time2)
  const isAfter = timeOne.isAfter(timeTwo)
  console.log({ isAfter })
  return isAfter
}

export function timeAdd(time, number, unit) {
  const newTime = dayjs.tz(time).add(number, unit)
  return newTime.toDate()
}

export function encodeId(id) {
  // return id;
  const b64 = CryptoJS.AES.encrypt(`${id}`, cryptoSalt).toString();
  const e64 = CryptoJS.enc.Base64.parse(b64);
  const result = e64.toString(CryptoJS.enc.Hex);
  return result;
}

export function decodeId(idString) {
  // return parseInt(idString);
  const reb64 = CryptoJS.enc.Hex.parse(idString);
  const bytes = reb64.toString(CryptoJS.enc.Base64);
  const decrypt = CryptoJS.AES.decrypt(bytes, cryptoSalt);
  const id = parseInt(decrypt.toString(CryptoJS.enc.Utf8));
  return id;
}

export function randomStr(length) {
  let result = '';
  let chars = '0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ'
  for (let i = length; i > 0; --i) {
    result += chars[Math.floor(Math.random() * chars.length)]
  }
  return result;
}

export function md5Hash(data) {
  const result = CryptoJS.MD5(data).toString()
  console.log('md5', result)
  return result
}


export function parseUrl(str) {
  const result = url.parse(str)
  return result
}

export function countBy(arr, field) {
  const result = loadsh.countBy(arr, field)
  return result
}

export function sumBy(arr, field) {
  const result = loadsh.sumBy(arr, field)
  return result
}

export function groupBy(arr, field) {
  const result = loadsh.groupBy(arr, field)
  return result
}
