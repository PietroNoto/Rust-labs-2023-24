use std::str;

const bsize: usize = 20;

pub struct Board 
{
    boats: [u8; 4],
    data: [[u8; bsize]; bsize],
}

pub enum Error 
{
    Overlap,
    OutOfBounds,
    BoatCount,
}

pub enum Boat 
{
    Vertical(usize),
    Horizontal(usize)
}


impl Board 
{
    /** crea una board vuota con una disponibilità di navi */
    pub fn new(boats: &[u8]) -> Board 
    {
        let mut boats_array: [u8; 4] = [0; 4];
        for i in 0..4
        {
            boats_array[i] = boats[i];
        }
        Board {boats: boats_array, data: [[' ' as u8; bsize]; bsize] }
    }


    /* crea una board a partire da una stringa che rappresenta tutto
    il contenuto del file board.txt */
    pub fn from(s: String) -> Board 
    {
        let lines = s.split("\n").collect::<Vec<&str>>();

        //4,2,1,2 -> ["4", "2", "1", "2"]
        let mut boats = [0 as u8; 4];
        for (i,b) in lines[0].split(",")
            .map(|s| s.parse::<u8>().unwrap()).enumerate()
        {
            boats[i] = b;
        }

        //["   b       ...", " bbb     bb  ...", ...]
        let mut data = [[' ' as u8; bsize]; bsize];
        let data_ = lines[1..20].iter()
            .map(|s|s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        for (i, arr) in data_.iter().enumerate()
        {
            for (j, ch) in arr.iter().enumerate()
            {
                data[i][j] = (*ch) as u8;
            }
        }
        
        Board {boats: boats, data: data}
    }


    /* aggiunge la nave alla board, restituendo la nuova board se
    possibile */
    /* bonus: provare a *non copiare* data quando si crea e restituisce
    una nuova board con la barca, come si può fare? */
    pub fn add_boat(&mut self, boat: Boat, pos: (usize, usize)) -> Result<Board, Error> 
    {
        //BoatCount, OutOfBounds, Overlap
        match boat
        {
            Boat::Vertical(length) => 
            {
                if self.boats[length - 1] <= 0
                {
                    return Err(Error::BoatCount);
                }
                else 
                {
                    let x = pos.0 - 1;
                    let y_start = pos.1 - 1;
                    for i in y_start..bsize
                    {
                        if i + length > bsize
                        {
                            return Err(Error::OutOfBounds);
                        }
                        for j in i..(i + length - 1)
                        {
                            if self.data[x][j] == 'B' as u8
                            {
                                return Err(Error::Overlap);
                            }
                        }
                        for j in i..(i + length - 1)
                        {
                            self.data[x][j] = 'B' as u8;
                        }
                    }
                }
            },
            Boat::Horizontal(length) => 
            {
                if self.boats[length - 1] <= 0
                {
                    return Err(Error::BoatCount);
                }
                else 
                {
                    let x_start = pos.0 - 1;
                    let y = pos.1 - 1;
                    for i in x_start..bsize
                    {
                        if i + length > bsize
                        {
                            return Err(Error::OutOfBounds)
                        }
                        for j in i..(i + length - 1)
                        {
                            if self.data[j][y] == 'B' as u8
                            {
                                return Err(Error::Overlap);
                            }
                        }
                        for j in i..(i + length - 1)
                        {
                            self.data[j][y] = 'B' as u8;
                        }
                    }
                }
            }
        }
        Ok(Board {boats: self.boats, data: self.data})
    }


    /* converte la board in una stringa salvabile su file */
    pub fn to_string(&self) -> String
    {
        let mut board_string = str::from_utf8(&self.boats).unwrap().to_string();
        board_string.push_str("\n");
        
        for line in 0..bsize
        {
            board_string.push_str(str::from_utf8(&self.data[line]).unwrap());
            board_string.push_str("\n");
        }   
        board_string
    }
}