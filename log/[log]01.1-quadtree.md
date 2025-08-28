
## Cara Kerja
1. setiap unit memiliki component QuadtreeUnit dimana itu akan membuat unit itu bisa diakses dalam pemasukan pada quadtree
2. Init Quadtree sebagai Resource ke bevy
3. Pemasukan unit ke dalam quadtree, dimana itu berisikan dengan langkah - langkah berikut
3.1. looping setiap unit
3.2. memeriksa apakah unit tersebut belum masuk ke dalam quadtree
3.3. jika belum maka masukkan dan jika sudah maka biarkan
3.4. cek perubahan pada posisi dari setiap anakan
4. kita dapat menggunakan quadtree tersebut kini terutama dalam pengecekan collision


## Masalah Yang Masih Saya Temukan
oke beberapa masalah yang masih ada dan tengah saya cari cara untuk menghadapinya adalah 
perpindahan dari satu grid ke grid lainnya

jadi saya memiliki beberapa solusi yang dapat kita gunakan:
1. daripada kita pusing bagaimana cara menaruh data di dalam quadtree itu langsung. kita akan menggunakan
pendekatan ecs itu sendiri dimana daripada kita benar - benar menaruh posisi itu ke dalam Res quadtree.
kita akan memasukkan entitynya saja daripada posisinya. dengan kata lain kita akan merefaktor quadtree 
itu secara sepenuhnya
