# UI Games
ui dari games Throwback Kingdoms itu sendiri dibagi menjadi beberapa bagian, bagian - bagian tersebut ialah
1. RPG Modes
    1. Slot Items (Bottom Center)
    2. Action Lists (Bottom End)
    3. MiniMap (Bottom Start)
    4. Healthbar, Energy, Status (Start Top)
    5. Sidebar Access (Bawahnya Healthbar)
    6. Operation Minipanel (Top End)



## RPG Modes
### Slot Items
Slot items itu sendiri diatur dengan Component **TkItemSlot** dan juga **Button** dimana button tersebut akan mengatur
Component RpgInventory


## Masalah
### Pemaparan Dan Solusi (Saat Ini)
Salah satu masalah yang mungkin belum dapat saya selesaikan atau pikirkan adalah masalah terkait Itemslot yang
dipilih saat dalam mode RPG dan efeknya pada mode RTS, serta bukankah apabila setiap unit memiliki ini akan
boros. metode untuk mengatasi hal tersebut yang ada dalam pikiran saya saat ini adalah dengan hal berikut:
1.  Mode RTS tidak menggunakan konsep Inventory Slot tapi lebih ke general inventory. jadi tidak peduli apabila
    kau memilih items tertentu, selama ada barang itu di inventory make dapat dilakukan **[General Inventory Sys]**
2.  Tidak semua Unit memiliki system inventory tersebut seperti RPG, hanya heroes yang memiliki system tersebut
3.  Apabila Mau, Kita dapat menghapus system Pemilihan secara keseluruhan dan menggunakan General Inventory Sys
    pada seluruh Game. dan untuk menyerang kita hanya perlu menekan tombol Toggle yang mengubah semua aksi kita
    menjadi aksi menyerang

### Masukan
1.  RPG mode juga memperbolehkan Penggunaan General Inventory Sys (Seperti ketika hendak menebang pohon)
