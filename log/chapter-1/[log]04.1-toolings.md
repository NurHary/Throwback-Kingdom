# Inventory Systems
_Deskripsi: Sistem penggunaan tool baik untuk karakter rpg ataupun rts_
_Inspirasi utama sistem:_
    _1.  Last Days On Earth: Sistem dimana tool ada di inventory dan ada satu tombol yang justru bekerja dengan memanfaatkan tool yang ada_
        _di Inventory untuk all purpose_


## Main Line
1.  Sistem Penggunaan Tools
2.  Sistem Update Tools
3.  Considerations

## 1. Sistem Penggunaan Tools
Pertama - tama, kita lakukan perumusan tools dan sistem untuk tools tersebut
1.  Axe     :   Untuk menebang pohon; pohon akan terus tumbuh seiring waktu di daratan dunia
2.  Pickaxe :   Untuk menambang batu; bisa ditemukan di dunia tapi tidak tumbuh atau tumbuh tapi di tambang / dungeon
3.  Hammer  :   Untuk membangun bangunan, memperbaiki bangunan, dan untuk cek bangunan
4.  Hoe     :   Untuk membajak sawah yang digunakan untuk bertani
5.  W Can   :   Untuk menyiram tanaman yang di tanam
6.  Sickle  :   Untuk harvest tanaman
7.  Weapons :   Untuk bertarung

dengan enam jenis tools tersebut, maka juga diperlukan dua hal berikut:
1.  Workshop    :   Tempat untuk membuat peralatan tersebut (kecuali yang primitif dimana primitif dapat dicraft langsung tanpa syarat)
2.  Tool Holders:   Tempat untuk menyimpan peralatan yang telah dibuat

_terkhusus untuk weapons_:
weapons di TK bekerja berbeda dengan yang lainnya dimana weapons harus di unsheathe dari karakter dengan menggunakan tombol B (untuk rpg) dan
select char lalu tekan B (battle mode / pesiapan pertarungan) di mode rts.

selain itu juga, weapon di Tk juga memiliki slot khusus dimana juga weapon di tk juga akan memiliki opsi untuk membuka skill dengan melakukan
gerakan kombo tertentu

# 2. Sistem Update Tools
ada dua opsi yang dapat dilakukan dalam sistem update tools disini

## 3. Considerations
Ada beberapa sistem yang dapat di considerate untuk sistem tooling, yaitu:

### 1. Sistem Durability Tool
On paper terdengar sangat bagus dimana dengan begini pemain harus terus membuat peralatan mereka karena peralatan mereka dapat hancur.
tapi mungkin ini bisa menjadi cukup kurang baik dikarenakan dengan begitu resource yang dibutuhkan harus meningkat secara linear atau expo
berdasarkan jumlah unit

#### Keputusan
**APPROVED**
Sistem ini akan ditambahkan dikarenakan dengan begitu dapat meningkatkan strategi dan tingkat kesusahan pada game tersebut dengan catatan tambahan
tambahkan di sistem init / world generation untuk disable atau enable sistem ini
