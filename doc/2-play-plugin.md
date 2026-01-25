# play_plugin
Plugin play_plugin adalah plugin utama yang mengatur state dari gameplay (GameState::Play) itu sendiri dimana didalam
plugin tersebut menampung beragam fungsi - fungsi yang dipisahkan / dijalankan berdasarkan game mode saat ini (RPG/RTS)

Fungsi - Fungsi Tersebut adalah sebagai berikut
1. Startup
    1. spawn_character (fungsi untuk menginisialisasi karakter)
    2. setup_camera (Fungsi untuk menginisiasi system kamera [bevy_pancam])
2. Update
    1. maingameloop (untuk melakukan operasi pengubahan mode dan lainnya)
    2. handle_rpg_slot (untuk system ui inventory slot) [NOTES: Posisi akan diganti ke dalam tkgameui itu sendiri sebagai plugin mandiri]
    3. rpg_play (fungsi yang menjalankan mode rpg secara utuh)
    4. rpg_camrea_movement (fungsi untuk menghandle pergerakan kamera dari heroes rpg)
    5. rts_play (fungsi untuk menjalankan mode rts, untuk saat ini hanya ada kegunaan untuk melakukan marquee select)
    6. rts_handle_movement (fungsi untuk menghandle movement unit yang terpilih dalam mode rts)

