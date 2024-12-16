pada state ini entity itu berupa suatu struct (oop) yang memiliki field:
  posisi x: x
  posisi y: y
  movement speed: speed
  tipe character: Enum (CharacterTypes)



// Camera

  sekarang kita akan memasuki sesi pembuatan kamera, dimana saya ingin kamera dapat berganti menjadi 2 pov
  pov karakter utama dan pov RTS. tentu saya harus melakukan hal ini sebelumnya

  1. set camera mengikuti character utama terpilih dengan smoothing ### selesai
  2. zoom in zoom out untuk pov rts ### selesai
  3. menggerakkan kamera rts dengan mouse (control bind dan pergerakan mouse)

    hal yang harus diperhatikan: 
    1. system zoom in zoom out (nilai zoom in zoom out dan control mousenya scroll),
      kita butuh clamp nilai
    2. System Panning ## setengah selesai, kurang pada smoothingnya saja dimana
      kita harus membuat itu terasa seperti alat vektor
// Collision

  sekarang kita akan memasuki sesi collision
  beberapa hal yang perlu di tulis adalah:

  1. Physic collision
  2. area collision
  3. signal
