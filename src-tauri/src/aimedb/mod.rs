use std::io::{Cursor, Seek, SeekFrom};

use aes::cipher::{block_padding::ZeroPadding, BlockDecryptMut, BlockEncryptMut, KeyInit};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod key;

fn encrypt(data: &[u8]) -> Vec<u8> {
    let encryptor = ecb::Encryptor::<aes::Aes128Enc>::new(key::KEY.into());
    encryptor.encrypt_padded_vec_mut::<ZeroPadding>(data)
}

fn decrypt(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let decryptor = ecb::Decryptor::<aes::Aes128Dec>::new(key::KEY.into());
    Ok(decryptor.decrypt_padded_vec_mut::<ZeroPadding>(data)?)
}

async fn get_buf(len: u16, ty: u16) -> anyhow::Result<Cursor<Vec<u8>>> {
    let mut buf = Cursor::new(vec![0; len as usize]);
    buf.write_u32(0x3ea18730).await?;
    buf.write_u16_le(ty).await?;
    buf.write_u16_le(len).await?;
    Ok(buf)
}

pub async fn get_aimeid(host: &str, access_code: &str) -> anyhow::Result<i32> {
    let mut stream = tokio::net::TcpStream::connect(format!("{}:22345", host)).await?;
    let mut buf = get_buf(0x30, 0x0f).await?;
    buf.seek(SeekFrom::Start(0x20))?;
    buf.write_all(&hex::decode(access_code)?).await?;

    stream.write_all(&encrypt(buf.get_ref())).await?;
    stream.flush().await?;

    stream.readable().await?;
    let mut buf = vec![];
    stream.read_buf(&mut buf).await?;
    let mut buf = Cursor::new(decrypt(&buf)?);
    buf.seek(SeekFrom::Start(0x20))?;
    Ok(buf.read_i32_le().await?)
}
