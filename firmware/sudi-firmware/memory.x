MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* You must fill in these values for your application */
  
  /* included soft device */
  FLASH : ORIGIN = 0x00000000 + 156K, LENGTH = 512K - 156K
  RAM : ORIGIN = 0x20000000 + 44K , LENGTH = 64K - 44K
}
