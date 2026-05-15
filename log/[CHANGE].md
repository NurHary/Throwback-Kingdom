
# 13-05-2026
- Menambahkan sistem morton code dan juga depth id pada quadtree
- Menambahkan implementasi untuk mendapatkan partisi berdasarkan id

# 12-05-2026
- Mengubah sistem distribute dan sistem delete

# 08-05-2026
- Fuck Aku lupa edit ini pada tanggal 07
-   Refactor pada bagian tkquadtree, sekarang distribute yang akan selalu digunakan oleh fungsi to_qt dan update sedangkan
    insert hanya dipanggil dari dalam distribute
- Fix Collision pattern dimana unit to unit akan saling dorong sedangkan lainnya akan berhenti / pass through

# 06-05-2026
- Menambahkan Fungsi Cek dan Rekursif untuk insert items pada slots
- Menambahkan Struct healthpoint dengan implementasinya
- Menambahkan prototype tkobjects
- Menambahkan protoype untuk animasi objects
- rename TkAnimation menjadi TkUnitAnimation

# 22-03-2026
- sebenarnya ada banyak perubahan tapi lupa tidak di catat
- mengubah system spawn items dimana kini menggunakan macro
- mengubah system / cara kerja render ui untuk rpg
- memperbaiki beragam bug / error logika di tkitems dan tkinventory
- clear ui ketika ganti karakter
- insert item ke dalam inventory via collisions
- Clear layar ketika ganti mode

# 19-03-2026
- Mengembalikan fungsi untuk membangun game ui lebih sepsifik untuk membuat inventory slot, tapi kini mengikuti karakter inventory slot

# 29-02-2026
- Memperbaiki fungsi check items di tkitems

# 27-02-2026
- Change DynamicHeroesId from linklist to for i vector
- Menambahkan fungsi split data di tkitems
- Menambahkan fungsi append_to_items dan append_to_slots di tkinventory
- Mengganti fungsi check_contains_items untuk sekarang return dua option dua unsigned int untuk index dan stack sisa
