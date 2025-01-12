pada state ini entity itu berupa suatu struct (oop) yang memiliki field:
  posisi x: x
  posisi y: y
  movement speed: speed
  tipe character: Enum (CharacterTypes)



// Camera ## Separuh selesai

  sekarang kita akan memasuki sesi pembuatan kamera, dimana saya ingin kamera dapat berganti menjadi 2 pov
  pov karakter utama dan pov RTS. tentu saya harus melakukan hal ini sebelumnya

  1. set camera mengikuti character utama terpilih dengan smoothing ### selesai
  2. zoom in zoom out untuk pov rts ### selesai
  3. menggerakkan kamera rts dengan mouse (control bind dan pergerakan mouse) // Selesai

    hal yang harus diperhatikan: 
    1. system zoom in zoom out (nilai zoom in zoom out dan control mousenya scroll),
      kita butuh clamp nilai
    2. System Panning ## bagian panning selesai, sekarang bagian menghentikannya saja
      !!kita harus membuat itu terasa seperti alat vektor
      
// Collision #selesai

  sekarang kita akan memasuki sesi collision
  beberapa hal yang perlu di tulis adalah:

  1. Physic collision
  2. area collision
  3. signal

    // untuk colision sudah bekerja, yang kurang adalah disaat karakter bersentuhan maka karakter
      itu akan langsung ter assign

  // UI

  1. Benchmarking
    Bagaimana cara menggambar text yang mengikuti
  2. 
