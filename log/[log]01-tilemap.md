# Tilemap
## Bagaiamana cara Menstore data dan melakukan pengecekan collision
### data tanah dan particle layer
untuk data dari tanah yang akan kita gunakan itu sendiri kita akan menggunakan konsep dasar seperti hanya mengguanakan Vec seperti biasanya
### data collision object and breakable dan building
untuk data ini itu sendiri kita akan mengguanakan data struktur quadtree


- layer Omnni: Character Level
- layer 3: Object Level
- layer 2: Particle Level
- layer 1: Biome Level
- layer 0: Ground Level

: jadi kita akan memisahkan antara quadtree dengan tilemap itu sendiri dimana quadtree akan menyimpan seluruh data tentang collision sedangkan 
: tilemap akan menyimpan data seperti sprite dari tiles tersebut dalam bentuk index atlas
: jadi karakter ataupun lainnya tidak akan di masukkan dalam tilemap

apakah kita menyimpan suatu data di quadtree dalam model rect atau cukup vec2 saja?

model 1 Tilemap:
- for i in tilemap: spawn sprite, pos_index, tile_index
- operasi yang dilakukan pada dunia akan membuat tilemap bertindak dimana itu akan mengubah index pada tilemap
// sepertinya ini tidak efisien

Tilemap skip dulu, kita fokus pada bagian quadtree terlebih dahulu
alasan: Rendering dengan model batch rendering
