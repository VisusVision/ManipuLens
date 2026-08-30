---
title: "Ajan ve nihai rapor veri sözleşmesi"
order: 14
parca: model
zorluk: baslangic
dosya: "src/types.rs"
aralik: [14, 29]
dil: rust
ne_yapiyor: "Bir uzman cevabında ve kullanıcıya dönen nihai raporda hangi alanların bulunacağını Rust türleriyle zorunlu kılar."
neden_boyle: "Serbest metin sonraki katmanların neyi nerede bulacağını belirsiz bırakır. Ortak tür, ajanları karşılaştırmayı, JSON üretmeyi ve uzantının sonucu güvenle okumasını aynı sözleşmeye bağlar."
kaldirirsak: "Her ajan farklı biçimde cevap verebilir; sentezör güven puanını veya hedef cümleyi güvenilir biçimde bulamaz. Alan adı hataları ancak kullanıcı çalıştırdığında ortaya çıkar."
notlar:
  - satirlar: [1, 7]
    metin: "`AgentAnalysis` beş zorunlu alan taşır. `Vec<String>` bir ajanın sıfır, bir veya birden çok hedef cümle döndürebileceğini açıkça söyler."
  - satirlar: [9, 16]
    metin: "`FinalReport`, genel hüküm ile `detailed_analyses` kanıt listesini ayırır. Arayüz kısa cevap gösterebilir ama ayrıntıyı kaybetmez."
  - satirlar: [1, 9]
    metin: "İki yapı da hem `Serialize` hem `Deserialize` türetiyor: Rust'tan JSON'a yazılabilir ve model/istemci JSON'undan tekrar okunabilir."
sina:
  - soru: "`target_sentences` neden tek bir `String` değil?"
    cevap: "Bir manipülasyon tek cümleyle sınırlı olmayabilir; liste sıfır veya birden çok kanıtı aynı sözleşmeyle taşır."
  - soru: "Tür tanımı modelin doğru analiz yaptığını garanti eder mi?"
    cevap: "Hayır. Yalnız biçimi garanti eder; güven ve kanıtın anlamlı olup olmadığını sentezör ve uygulama kuralları denetler."
owner: TODO
needs_reverify: true
---

Model çıktısının çevresindeki en önemli kod bazen model çağrısı değil, çıktının sığmak zorunda olduğu türdür. Şema ne kadar darsa hata o kadar erken ve görünür olur.
