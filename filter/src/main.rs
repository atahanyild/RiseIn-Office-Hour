//filter condition struct'ını oluşturuyoruz
//<T> şeklinde değişkeni sonradan belirleyeceğimizi belirterek filter'ı T değişkenine eşitliyoruz
struct FilterCondition<T>{
    filter : T,
}

//impl ile struct'ımıza fonksiyon ekliyoruz.
//PartialOrd: < > <= >= şeklinde karşılaştırılabilir değişken olduğunu belirtiyor. integer ve float veri yapısına uygun 
impl<T: PartialOrd> FilterCondition <T> {
    fn is_match(&self,item: &T) -> bool {
        item > &self.filter
    }
}

//custom filter fonksiyonu ile list'in içerisinde dolaşarak tüm değişkenleri filter değeriyle karşılaştırıyoruz.
fn custom_filter<T>(list: Vec<T> , condition: &FilterCondition<T>) -> Vec<T> where T: PartialOrd {
    return list.into_iter().filter(|item: &T| condition.is_match(item)).collect();
}

fn main(){
    //vector oluşturuyoruz. integer ve float ile kullanabiliriz.
    let numbers = vec![5.0,6.0,10.0,15.0,20.0];
    let condition = FilterCondition{filter: 5.0};

    let filtered_list = custom_filter(numbers, &condition);
    println!("{:?}" , filtered_list);
}


