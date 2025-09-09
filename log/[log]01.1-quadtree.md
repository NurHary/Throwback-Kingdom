
## Cara Kerja
1. setiap unit memiliki component QuadtreeUnit dimana itu akan membuat unit itu bisa diakses dalam pemasukan pada quadtree
2. Init Quadtree sebagai Resource ke bevy
3. Pemasukan unit ke dalam quadtree, dimana itu berisikan dengan langkah - langkah berikut
3.1. looping setiap unit
3.2. memeriksa apakah unit tersebut belum masuk ke dalam quadtree
3.3. jika belum maka masukkan dan jika sudah maka biarkan
3.4. cek perubahan pada posisi dari setiap anakan
4. kita dapat menggunakan quadtree tersebut kini terutama dalam pengecekan collision

## Komponen Fungsi
### unit to quadtree
ini adalah komponenen fungsi yang digunakan untuk memasukkan setiap entity dengan component quadtree untuk masuk ke dalam quadtreenya itu sendiri

dimana cara kerja dari Komponen Fungsi ini adalah sebagai berikut:
- Bevy melakukan run system dengan schedule update dimana parameternya adalah objek dengan Added<QuadtreeUnit>
- dengan Added<QuadtreeUnit> maka setiap entity yang memiliki component itu hanya akan di panggil sekali yang kemudian
- kita dapat memanfaatkan itu untuk memasukkan setiap unit ke dalam quadtree tersebut

### update to quadtree
ini adalah komponen fungsi yang digunakan untuk memindahkan suatu entity dalam suatu partition quadtree ke pertition lainnya

dimana cara kerja dari komponen fungsi ini adalah sebagai berikut:
- fungsi dengan parameter Changed<Transform> dengan begitu kita dapat melakukan pengecekan apakah entity yang memiliki transform mengalami perpindahan
- 

### delete partition
### distribute partition
