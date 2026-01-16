# Inventory System dalam Throwback Kingdoms
jadi untuk sistem inventory, saya berniat untuk **menggunakan satu Komponen sebagai tempat penyimpanan
dan satu komponen lagi untuk memberikan indikasi jarak akses dari inventory tersebut (bisa antara
diganti dengan Enum atau murni itu saja, menurutku pakai bentuk komponen saja karena bisa menghemat
memori karena yang dilakukan proses hanyalah yang memilikinya)**

## Component
dengan kata lain, akan ada 2 struct component:
1. TkInventory -> Lebih gampang dibuat untuk saat ini (megnecualikan insert)
2. TkSharedInventory -> Lebih susah karena memiliki area collision

jadi pertama tentu kita harus mengetahui apa saja operasi yang harus dimiliki oleh component ini.
jadi berikut adalah daftar operasi tersebut:

### Cara Kerja
Lorem Ipsum Dolor Sit Amet

## TkInventory Operation
### 1. insert ke inventory
Cara kerja adalah
### 2.


## TkSharedInventory Operation
### 1. Pengecekan Collision Unit yang ada Didalam Area
Cara Kerjanya adalah dengan memanfaatkan Quadtree, disini kita akan mengecek di partisi mana SharedInventory
ini berada
### 2. Extend Inventory dari semua unit yang ada dalam area
