# Main
## Plugin
main.rs menggunakan Bevy dengan model modular dimana sistem - sistem dari game itu sendiri akan di pisah dengan menggunakan model modular plugin itu sendiri

untuk saat ini terdapat beberapa plugin yang telah dibuat itu sendiri yaitu:

saat ini:

GameplayPlugin
|   play    // Fungsi yang dijalankan ketika play dimana ini sifatnya global tidak peduli kau dalam mode apa
|   play_rts (if rc_gamemode) // Fungsi yang dijalankan ketika kau ada di dalam model rts
|   play_rpg (if !rc_gamemode) // Fungsi yang dijalankan ketika kau ada di dalam model rpg

TkQuadTreePlugin
|   unit_to_quadtree    // Fungsi yang akan menambahkan entity baru dengan componen QuadtreeUnit kedalam TkQuadTree
|   update_quadtree_unit    // Fungsi yang akan melakukan update pada quadtree ketika ada perpindahan suatu objek dari 1 quadtree ke quadtree lainnya
|   

target ke depan
MainMenuPlugin


## Resource
di project ini terdapat beberapa resource yang telah digunakan yaitu berupa berikut:

TkQuadTree
|   tile
|   boundary
{impl
|   insert(en, tr)          // Fungsi yang digunakan untuk menaruh sesuatu kedalam Quadtree tersebut fungsi ini membutuhkan entity (en) yang akan dimasukkan dan posisi (tr) untuk menentukan 
|   subdivide               // Fungsi yang digunakan ketika jumlah anakan dalam suatu quadtree lebih dari empat, fungsi ini akan membuat children quadtree berdasaarkan parameter quadtree itu sendiri dan 
|   distribute
|   contains
|   get_partition
|   check_entity
|   ray_partition
