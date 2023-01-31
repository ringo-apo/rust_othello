extern crate rand;

use std::io;
use rand::distributions::{Distribution, Uniform};

// 0:設定なし
// 1:コマなし
// 2:黒
// 3:白
// 4:置けるところ
// 5:枠外

static MUKI: [(i32, i32); 8] =
    [(-1, -1), (-1, 0), (-1, 1),
     ( 0, -1),          ( 0, 1),
     ( 1, -1), ( 1, 0), ( 1, 1)];


// コマを数える　ｘ方向
fn func28(bd: &Vec<Vec<i32>>, iro: i32, y: i32, x: i32, iro_num: i32) -> i32 
{
    let iro_num2 = 
        if bd[y as usize][x as usize] == iro 
        {
            let iro_num3 = iro_num + 1;
            iro_num3
        } 
        else 
        {
            iro_num
        };

    let iro_num4 = 
        if x == 1 
        {
            iro_num2
        } 
        else 
        {
            let iro_num5 = func28(bd, iro, y, x-1, iro_num2);
            iro_num5
        }; 

    iro_num4
}


// コマを数える　ｙ方向
fn func27(bd: &Vec<Vec<i32>>, iro: i32, y: i32, iro_num: i32) -> i32
{
    let iro_num2 = func28(&bd, iro, y, 8, iro_num);

    let iro_num3 = 
        if y == 1
        {
            iro_num2                
        } 
        else 
        {
            let iro_num4 = func27(bd, iro, y-1, iro_num2);
            iro_num4
        };

    iro_num3
} 


// main_loop2
fn func25(bd: Vec<Vec<i32>>, teban: i32, aite: i32, hum: i32, com: i32, ban: i32, pass: i32) -> (Vec<Vec<i32>>, i32)
{
    let (bd2, pass2) =
        if teban == hum
        {
            let (bd3, pass3) = func22(bd, teban, aite, hum, com, pass);
            (bd3, pass3)
        }
        else
        {
            let (bd4, pass4) = func23(bd, teban, aite, hum, com, pass);
            (bd4, pass4)
        };

    let (bd5, pass5) =
        if pass2 >= 2
        {
            (bd2, pass2)
        }
        else
        {
            let (teban2, aite2) =
                if teban == 2
                {
                    println!("白番です");
                    (3, 2)
                }
                else
                {
                    println!("黒番です");
                    (2, 3)
                };

            let (bd6, pass6) = func25(bd2, teban2, aite2, hum, com, ban, pass2);
            (bd6, pass6)
        };
    (bd5, pass5)
}


// comの番 teban = com, aite = human
fn func23(bd3: Vec<Vec<i32>>, teban2: i32, aite2: i32, _hum: i32, _com: i32, pass: i32) -> (Vec<Vec<i32>>, i32)
{

    let okeru_list2: Vec<(usize, usize, i32)> = Default::default();

    let okeru_list3 = func4(&bd3, &teban2, &aite2, &8, &8, okeru_list2);

// 盤面変更
    let okeru_list3_len2: i32 = (okeru_list3.len() as i32)-1;

    let (bd5, pass3) =
        if okeru_list3_len2 == -1
        {
            let pass4 = pass + 1;
            println!("パス");
            (bd3, pass4)
        }
        else
        {

            let _bd4: Vec<Vec<i32>> = func7(&bd3, &okeru_list3, okeru_list3_len2);

            func21(&okeru_list3, okeru_list3_len2);

            let mut rng = rand::thread_rng();
            let okibasyo_num = Uniform::from(0..okeru_list3_len2+1);
            let throw = okibasyo_num.sample(&mut rng);
            println!("{}", throw);
            let okibasyo2: (usize, usize, i32) = (okeru_list3[throw as usize].0, okeru_list3[throw as usize].1, teban2);

// ひっくり返し
            let y: i32 = okibasyo2.0 as i32;
            let x: i32 = okibasyo2.1 as i32;

            let henko_list3: Vec<(usize, usize, i32)> = vec![(y as usize, x as usize, teban2)];

            let muki_len2: i32 = (MUKI.len() as i32)-1;

            let henko_list4: Vec<(usize, usize, i32)> = func20(&bd3, muki_len2, henko_list3, y, x, teban2, aite2);

            let henko_list4_len2: i32 = (henko_list4.len() as i32)-1;

            let bd4: Vec<Vec<i32>> = func7(&bd3, &henko_list4, henko_list4_len2);

            (bd4, 0)

        };
    func1(&bd5);

    (bd5, pass3)
}


// 人の番 teban = human, aite = com
fn func22(bd: Vec<Vec<i32>>, teban: i32, aite: i32, _hum:i32, _com: i32, pass: i32) -> (Vec<Vec<i32>>, i32)
{

   // 右下から操作を開始する。再帰を使うので戻り値の変数も引数に含める
    let okeru_list0: Vec<(usize, usize, i32)> = Default::default();

    let okeru_list = func4(&bd, &teban, &aite, &8, &8, okeru_list0);

    let (bd4, pass3) =
        if okeru_list.len() == 0
        {
            let pass4 = pass + 1;
            println!("パス");
            (bd, pass4)
        }
        else
        {
// 盤面変更
            let okeru_list_len2: i32 = (okeru_list.len()-1).try_into().unwrap();
            let bd2: Vec<Vec<i32>> = func7(&bd, &okeru_list, okeru_list_len2);
            func1(&bd2);

// 置き場所入力
// 置けるリストを表示
            let okibasyo: (usize, usize, i32) = func16(okeru_list, okeru_list_len2);

// ひっくり返し
            let y: i32 = okibasyo.0 as i32;
            let x: i32 = okibasyo.1 as i32;

            let henko_list: Vec<(usize, usize, i32)> = vec![(y as usize, x as usize, teban)];

            let muki_len2: i32 = (MUKI.len() as i32)-1;

            let henko_list2: Vec<(usize, usize, i32)> = func20(&bd, muki_len2, henko_list, y, x, teban, aite);
            let henko_list2_len2: i32 = (henko_list2.len() as i32)-1;
            let bd3: Vec<Vec<i32>> = func7(&bd, &henko_list2, henko_list2_len2);
            func1(&bd3);
            (bd3, 0)
        };

    (bd4, pass3)
}


// 置けるリストを表示 com
fn func21(okeru_list: &Vec<(usize, usize, i32)>, okeru_list_len: i32)
{
    println!("下記から選んでください");
    func17(&okeru_list, okeru_list_len);
}


// 向きごとの調査
fn func20(bd: &Vec<Vec<i32>>, muki_len2: i32, henko_list: Vec<(usize, usize, i32)>, y: i32, x: i32, teban: i32, aite: i32) -> Vec<(usize, usize, i32)>
{
    let y2: i32 = y + MUKI[muki_len2 as usize].0;
    let x2: i32 = x + MUKI[muki_len2 as usize].1;
    let atai: i32 = bd[y2 as usize][x2 as usize];
    let henko_list2: Vec<(usize, usize, i32)> =
        if atai == aite // aite 白
        {
            let henko_list_list: Vec<(usize, usize, i32)> = Default::default();
            let henko_list_list2: Vec<(usize, usize, i32)> = func19(&bd, henko_list_list, muki_len2, y2, x2, teban, aite);
            let henko_list3: Vec<(usize, usize, i32)> = henko_list.iter().cloned().chain(henko_list_list2.iter().cloned()).collect();

            let henko_list4: Vec<(usize, usize, i32)> =
                if muki_len2-1 < 0
                {
                    henko_list3
                }
                else
                {
                    let henko_list5: Vec<(usize, usize, i32)> = func20(bd, muki_len2-1, henko_list3, y, x, teban, aite);
                    henko_list5
                };
            henko_list4
        }
        else    // next
        {
            let henko_list6: Vec<(usize, usize, i32)> =
                if muki_len2-1 < 0
                {
                    henko_list
                }
                else
                {
                    let henko_list7: Vec<(usize, usize, i32)> = func20(bd, muki_len2-1, henko_list, y, x, teban, aite);
                    henko_list7
                };
            henko_list6
        };
    henko_list2
}


// 長さの調査。黒を探す
fn func19(bd: &Vec<Vec<i32>>, henko_list_list: Vec<(usize, usize, i32)>, muki_len2: i32, y2: i32, x2: i32, teban: i32, aite: i32) -> Vec<(usize, usize, i32)>
{
    let y3: i32 = y2 + MUKI[muki_len2 as usize].0;
    let x3: i32 = x2 + MUKI[muki_len2 as usize].1;
    let atai: i32 = bd[y3 as usize][x3 as usize];

    let henko_list_list2: Vec<(usize, usize, i32)> =
        if atai == teban // 黒
        {
            let henko_list_list4 = vec![(y2 as usize, x2 as usize, teban)];
            let henko_list_list3: Vec<(usize, usize, i32)> = henko_list_list.iter().cloned().chain(henko_list_list4.iter().cloned()).collect();
            henko_list_list3
        }
        else if atai == aite // 白
        {
            let henko_list_list4: Vec<(usize, usize, i32)> =
                if (y3 + MUKI[muki_len2 as usize].0 < 1) || (y3 + MUKI[muki_len2 as usize].0 > 8) || (x3 + MUKI[muki_len2 as usize].1 < 1) || (x3 + MUKI[muki_len2 as usize].1 > 8)
                {
                    let henko_list_list5: Vec<(usize, usize, i32)> = Default::default();
                    henko_list_list5
                }
                else
                {
                    let henko_list_list8: Vec<(usize, usize, i32)> = vec![(y2 as usize, x2 as usize, teban)];
                    let henko_list_list6: Vec<(usize, usize, i32)> = henko_list_list.iter().cloned().chain(henko_list_list8.iter().cloned()).collect();

                    let henko_list_list7: Vec<(usize, usize, i32)> = func19(&bd, henko_list_list6, muki_len2, y3, x3, teban, aite);
                    henko_list_list7
                };
            henko_list_list4
        }
        else
        {
            // next
            let henko_list_list8: Vec<(usize, usize, i32)> = Default::default();
            henko_list_list8
        };
    henko_list_list2
}

//
fn func18(okeru_list_len: i32) -> i32
{
    let mut input = String::new();

    let input_s: String =
        match io::stdin().read_line(&mut input)
        {
            Ok(_) =>
            {
                input
            },
            Err(_) => "-1".to_string(),
        };

    let input_r: Result<i32, _> = input_s.trim().parse();

    let input_n: i32 =
        match input_r
        {
            Ok(value) =>
            {
                value
            },
            Err(_) =>
            {
                let input_n2: i32 = func18(okeru_list_len);
                input_n2
            },
        };

    let input_n3 = if input_n >= 0 && input_n <= okeru_list_len
    {
        input_n
    }
    else
    {
        let input_n4 = func18(okeru_list_len);
        input_n4
    };
    input_n3
}

// 置けるリストを表示２
fn func17(okeru_list: &Vec<(usize, usize, i32)>, okeru_list_len: i32)
{
    let okeru_list_len2: usize = okeru_list_len as usize;
    let (data1, data2, _) = okeru_list[okeru_list_len2];
    let data2_2 =
        match data2
        {
            1 => "a",
            2 => "b",
            3 => "c",
            4 => "d",
            5 => "e",
            6 => "f",
            7 => "g",
            8 => "h",
            _ => "z",
        };
    println!("{}: {}{} ", okeru_list_len, data2_2, data1);

    if okeru_list_len == 0
    {
        println!();
    }
    else
    {
        func17(okeru_list, okeru_list_len-1);
    };
}


// 置けるリストを表示
fn func16(okeru_list: Vec<(usize, usize, i32)>, okeru_list_len: i32) -> (usize, usize, i32)
{
    println!("下記から選んでください");

    func17(&okeru_list, okeru_list_len);

    let num: i32 = func18(okeru_list_len);
    let num_u: usize = num as usize;
    let okibasyo = okeru_list[num_u];

    okibasyo
}

// 置けるリストを盤面に反映させる
fn func7(bd: &Vec<Vec<i32>>, okeru_list: &Vec<(usize, usize, i32)>, okeru_list_len: i32) -> Vec<Vec<i32>>
{
    let bd2: Vec<Vec<i32>> =
        if okeru_list_len >= 0
        {
            let okeru_list_len2: usize = okeru_list_len as usize;

            let cp: Vec<Vec<i32>> = (&bd[..okeru_list[okeru_list_len2].0]).to_vec();
            let cb: Vec<Vec<i32>> = (&bd[(okeru_list[okeru_list_len2].0+1)..]).to_vec();
            let d: Vec<i32> = (&bd[okeru_list[okeru_list_len2].0]).to_vec();
            let dp: Vec<i32> = (&d[..okeru_list[okeru_list_len2].1]).to_vec();
            let db: Vec<i32> = (&d[(okeru_list[okeru_list_len2].1+1)..]).to_vec();
            let data: Vec<i32> = vec![okeru_list[okeru_list_len2].2];
            let d2: Vec<&i32> = dp.iter().chain(data.iter()).chain(db.iter()).collect::<Vec<&i32>>();
            let d22: Vec<i32> = d2.into_iter().cloned().collect();
            let d222: Vec<Vec<i32>> = vec![d22];
            let bd3: Vec<&Vec<i32>> = cp.iter().chain(d222.iter()).chain(cb.iter()).collect();

            let bd32: Vec<Vec<i32>> = bd3.into_iter().cloned().collect();
            let bd3 = func7(&bd32, okeru_list, okeru_list_len-1);
            bd3
        }
        else
        {
            bd.to_vec()
        };
    bd2
}


// 隣の隣を調べる。その先に自分と同じ色のコマがあるか調べる
fn func6(hoko: i32, bd: &Vec<Vec<i32>>, y: usize, x: usize, teban: i32, aite: i32) -> bool
{
    let y5: i32 = y as i32;
    let x5: i32 = x as i32;

    let y3: i32 = MUKI[hoko as usize].0 as i32;
    let x3: i32 = MUKI[hoko as usize].1 as i32;

    let y4: i32 = y3 + y5;
    let x4: i32 = x3 + x5;

    let y2: usize = y4 as usize;
    let x2: usize = x4 as usize;

    let kekka: bool =
        if y2<1 || y2>8 || x2<1 || x2>8
        {
            false
        }
        else
        {
            let atai = bd[y2][x2];

            let kekka: bool =
                if atai == teban
                {
                    true
                }
                else if atai == aite
                {
// 継続操作。更にその隣を調べる
                    let kekka2 = func6(hoko, bd, y2, x2, teban, aite);
                    kekka2
                }
                else
                {
                    false
                };
            kekka
        };
    kekka
}

// そこにコマがないのがわかったので、次に周りを調べる
fn func5(hoko: i32, bd: &Vec<Vec<i32>>, y: usize, x: usize, teban: i32, aite: i32) -> bool
{
    let y5: i32 = y as i32;
    let x5: i32 = x as i32;

    let y3: i32 = MUKI[hoko as usize].0 as i32;
    let x3: i32 = MUKI[hoko as usize].1 as i32;

    let y4: i32 = y3 + y5;
    let x4: i32 = x3 + x5;

    let y2: usize = y4 as usize;
    let x2: usize = x4 as usize;

// 番外？
    let kekka: bool =
        if y2<1 || y2>8 || x2<1 || x2>8   // 番外
        {
// next
            let kekka2: bool =
                if hoko-1 >= 0
                {
                    let kekka3: bool = func5(hoko-1, bd, y, x, teban, aite);
                    kekka3
                }
                else
                {
                    false
                };
            kekka2
        }
        else
        {
            let atai = bd[y2][x2];
            let kekka: bool =
                if atai == aite    // そこに相手のコマがあれば捜査継続
                {
                    let kekka2: bool = func6(hoko, &bd, y2, x2, teban, aite); // 隣の隣を調べる
                    let kekka3 =
                        if kekka2 == true
                        {
                            true
                        }
                        else
                        {
                            let kekka5: bool =
                                if hoko-1 >= 0
                                {
                                    let kekka4: bool = func5(hoko-1, bd, y, x, teban, aite);
                                    kekka4
                                }
                                else
                                {
                                    false
                                };
                            kekka5
                        };
                    kekka3
                }
                else
                {
// 次の向きを調べる
                    let kekka: bool =
                        if hoko < 1
                        {
// 処理終了。帰る。
                            false
                        }
                        else
                        {
// 次の向きを調べる
                            let kekka: bool = func5(hoko-1, bd, y, x, teban, aite);
                            kekka
                        };
                    kekka
                };
            kekka
        };
    kekka
}


// 置ける場所を探す
fn func4(bd: &Vec<Vec<i32>>, teban: &i32, aite: &i32, y: &usize, x: &usize, okeru_list: Vec<(usize, usize, i32)>) -> Vec<(usize, usize, i32)>
{
    let atai = bd[*y][*x];

// 置ければtrue、置けなければfalseを返す
    let okeru: bool =    // 戻り値はそこに置けるかどうかの真理値
        if atai == 1 // コマなしであればその先を調べる
        {
            let kekka: bool = func5(7, &bd, *y, *x, *teban, *aite);
            kekka
        }
        else
        {
            false
        };

// okeruがtrueならばその場所のy,xを記録、次を調べる。falseならそのまま
    let okeru_list2: Vec<(usize, usize, i32)> =
        if okeru == true
        {
            let y2: usize = *y;
            let x2: usize = *x;
            let okeru_list2: Vec<(usize, usize, i32)> = vec![(y2, x2, 4)];
            let okeru_list3: Vec<(usize, usize, i32)> = okeru_list.iter().cloned().chain(okeru_list2.iter().cloned()).collect();
            okeru_list3
        }
        else
        {
            okeru_list
        };

// 次の場所を調べる。次が左上ならokeru_listを返す

    let okeru_list3 =
        if (x-1) >= 1
        {
// 再帰で次を調べる
            let okeru_list = func4(bd, teban, aite, y, &(x-1), okeru_list2);
            okeru_list
        }
        else if (y-1) >= 1
        {
            let okeru_list = func4(bd, teban, aite, &(y-1), &8, okeru_list2);
            okeru_list
        }
        else
        {
            okeru_list2
        };

    okeru_list3
}


// 手番選択
fn func3() -> i32
{
    println!("手番を選んでください。　１：黒番（先手）、２：白番（後手）");

    let mut hitoteban_s = String::new();
    io::stdin().read_line(&mut hitoteban_s).expect("failed to read line");

    let hitoteban_opt = hitoteban_s.trim().parse::<i32>();

    let hitoteban =
        match hitoteban_opt
        {
            Ok(x) => x,
            Err(_) => 0,
        };

    let hitoteban2 =
        match hitoteban
        {
            1 => 2,
            2 => 3,
            _ => 0,
        };

    let hitoteban4 =
        if hitoteban2 == 2
        {
            2
        }
        else if hitoteban2 == 3
        {
            3
        }
        else
        {
            let hitoteban3 = func3();
            hitoteban3
        };
    hitoteban4
}


// 表示変換処理
fn func2(atai: i32) -> &'static str
{
    match atai
    {
        0 => "　",
        1 => "　",
        2 => "〇",
        3 => "●",
        4 => "！",
        5 => "　",
        _ => "　",
    }
}

// 盤面表示処理
fn func1(bd: &Vec<Vec<i32>>)
{
    println!("　　ａ　ｂ　ｃ　ｄ　ｅ　ｆ　ｇ　ｈ　Ｘ");
    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji1 = "１＊".to_string() + func2(bd[1][1]) + "＊"
                                     + func2(bd[1][2]) + "＊"
                                     + func2(bd[1][3]) + "＊"
                                     + func2(bd[1][4]) + "＊"
                                     + func2(bd[1][5]) + "＊"
                                     + func2(bd[1][6]) + "＊"
                                     + func2(bd[1][7]) + "＊"
                                     + func2(bd[1][8]) + "＊";
    println!("{}", hyouji1);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji2 = "２＊".to_string() + func2(bd[2][1]) + "＊"
                                     + func2(bd[2][2]) + "＊"
                                     + func2(bd[2][3]) + "＊"
                                     + func2(bd[2][4]) + "＊"
                                     + func2(bd[2][5]) + "＊"
                                     + func2(bd[2][6]) + "＊"
                                     + func2(bd[2][7]) + "＊"
                                     + func2(bd[2][8]) + "＊";
    println!("{}", hyouji2);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji3 = "３＊".to_string() + func2(bd[3][1]) + "＊"
                                     + func2(bd[3][2]) + "＊"
                                     + func2(bd[3][3]) + "＊"
                                     + func2(bd[3][4]) + "＊"
                                     + func2(bd[3][5]) + "＊"
                                     + func2(bd[3][6]) + "＊"
                                     + func2(bd[3][7]) + "＊"
                                     + func2(bd[3][8]) + "＊";
    println!("{}", hyouji3);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji4 = "４＊".to_string() + func2(bd[4][1]) + "＊"
                                     + func2(bd[4][2]) + "＊"
                                     + func2(bd[4][3]) + "＊"
                                     + func2(bd[4][4]) + "＊"
                                     + func2(bd[4][5]) + "＊"
                                     + func2(bd[4][6]) + "＊"
                                     + func2(bd[4][7]) + "＊"
                                     + func2(bd[4][8]) + "＊";
    println!("{}", hyouji4);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji5 = "５＊".to_string() + func2(bd[5][1]) + "＊"
                                     + func2(bd[5][2]) + "＊"
                                     + func2(bd[5][3]) + "＊"
                                     + func2(bd[5][4]) + "＊"
                                     + func2(bd[5][5]) + "＊"
                                     + func2(bd[5][6]) + "＊"
                                     + func2(bd[5][7]) + "＊"
                                     + func2(bd[5][8]) + "＊";
    println!("{}", hyouji5);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji6 = "６＊".to_string() + func2(bd[6][1]) + "＊"
                                     + func2(bd[6][2]) + "＊"
                                     + func2(bd[6][3]) + "＊"
                                     + func2(bd[6][4]) + "＊"
                                     + func2(bd[6][5]) + "＊"
                                     + func2(bd[6][6]) + "＊"
                                     + func2(bd[6][7]) + "＊"
                                     + func2(bd[6][8]) + "＊";
    println!("{}", hyouji6);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji7 = "７＊".to_string() + func2(bd[7][1]) + "＊"
                                     + func2(bd[7][2]) + "＊"
                                     + func2(bd[7][3]) + "＊"
                                     + func2(bd[7][4]) + "＊"
                                     + func2(bd[7][5]) + "＊"
                                     + func2(bd[7][6]) + "＊"
                                     + func2(bd[7][7]) + "＊"
                                     + func2(bd[7][8]) + "＊";
    println!("{}", hyouji7);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");

    let hyouji8 = "８＊".to_string() + func2(bd[8][1]) + "＊"
                                     + func2(bd[8][2]) + "＊"
                                     + func2(bd[8][3]) + "＊"
                                     + func2(bd[8][4]) + "＊"
                                     + func2(bd[8][5]) + "＊"
                                     + func2(bd[8][6]) + "＊"
                                     + func2(bd[8][7]) + "＊"
                                     + func2(bd[8][8]) + "＊";
    println!("{}", hyouji8);

    println!("　＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊＊");
    println!("Ｙ");
    println!("[　]:コマなし");
    println!("[〇]:黒");
    println!("[●]:白");
    println!("[！]:置けるところ");
}

//
fn main() {

    let hitoteban: i32 = func3();

    let (hum, com) =
        if hitoteban == 2
        {
            let hum: i32 = 2;
            let com: i32 = 3;
            println!("You: 黒, Computer: 白");
            (hum, com)
        }
        else
        {
            let hum: i32 = 3;
            let com: i32 = 2;
            println!("You: 白, Computer: 黒");
            (hum, com)
        };

    let bd: Vec<Vec<i32>> =
        vec![vec![5,5,5,5,5,5,5,5,5,5],
             vec![5,1,1,1,1,1,1,1,1,5],
             vec![5,1,1,1,1,1,1,1,1,5],
             vec![5,1,1,1,1,1,1,1,1,5],
             vec![5,1,1,1,2,3,1,1,1,5],
             vec![5,1,1,1,3,2,1,1,1,5],
             vec![5,1,1,1,1,1,1,1,1,5],
             vec![5,1,1,1,1,1,1,1,1,5],
             vec![5,1,1,1,1,1,1,1,1,5],
             vec![5,5,5,5,5,5,5,5,5,5]
                                      ];

    let teban: i32 = 2; // 黒
    let aite: i32 = 3; // 白

    let ban =
        if hum == 2
        {
            1
        }
        else
        {
            2
        };

    let pass = 0;
    let (bd2, _pass2) = func25(bd, teban, aite, hum, com, ban, pass);

    println!("終了");

    let kuro_num = func27(&bd2, 2, 8, 0);
    println!("黒：{}", kuro_num);

    let shiro_num = func27(&bd2, 3, 8, 0);
    println!("白：{}", shiro_num);

    if kuro_num>shiro_num 
    {
        println!("黒の勝ち");
    } 
    else if kuro_num<shiro_num 
    {
        println!("白の勝ち");
    } 
    else 
    {
        println!("引き分け");
    };
}
