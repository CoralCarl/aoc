use std::collections::{HashMap, VecDeque};

trait Map<T> {
    fn map_index<'a, 'b: 'a>(
        &mut self,
        map: &mut HashMap<&'a [u8], ModuleIndex>,
        key: &'b [u8],
    ) -> ModuleIndex;
}

impl<T> Map<T> for Vec<T>
where
    T: Default,
{
    fn map_index<'a, 'b: 'a>(
        &mut self,
        map: &mut HashMap<&'a [u8], ModuleIndex>,
        key: &'b [u8],
    ) -> ModuleIndex {
        *map.entry(key).or_insert_with(|| {
            self.push(T::default());
            self.len() as ModuleIndex - 1
        })
    }
}

const BROADCASTER: u8 = b'b';
const FLIPFLOP: u8 = b'%';
const CONJUNCTION: u8 = b'&';
const OUTPUT: u8 = 0;

fn parse(input: &[u8]) -> Vec<Module> {
    let mut modules: Vec<Module> = Vec::with_capacity(59);
    modules.push(Module::default());
    let mut mod_map: HashMap<&[u8], ModuleIndex> = HashMap::with_capacity(59);
    mod_map.insert(b"broadcaster", 0);

    let mut i = 0;
    while i < input.len() {
        let module_type = input[i];
        if module_type == FLIPFLOP || module_type == CONJUNCTION {
            i += 1;
        }
        let mut j = i + 1;
        while input[j] != b' ' {
            j += 1;
        }
        let name_idx = modules.map_index(&mut mod_map, &input[i..j]);
        i = j + 4;
        j += 5;
        let mut wired: Vec<ModuleIndex> = Vec::new();
        while input[j] != b'\n' {
            if input[j] == b',' {
                let wired_idx = modules.map_index(&mut mod_map, &input[i..j]);
                wired.push(wired_idx);
                i = j + 2;
                j += 3;
            } else {
                j += 1;
            }
        }
        let wired_idx = modules.map_index(&mut mod_map, &input[i..j]);
        wired.push(wired_idx);
        modules[name_idx as usize] = (module_type, wired);
        i = j + 1;
    }
    modules
}

pub fn part1(input: &[u8]) -> String {
    let modules = parse(input);

    let mut parents: Vec<Vec<ModuleIndex>> = vec![Vec::default(); 59];
    for (i, module) in modules.iter().enumerate() {
        for child in &module.1 {
            parents[*child as usize].push(i as ModuleIndex);
        }
    }
    let mut last_signals = vec![0u8; 59];
    let mut states = vec![0u8; 59];
    let mut signals: [u32; 2] = [1000, 0];

    for _ in 0..1000 {
        let mut pulses: VecDeque<(ModuleIndex, u8)> = VecDeque::new();

        pulses.push_back((0, 0));
        while let Some((receiver, pulse)) = pulses.pop_front() {
            let (module_type, children) = &modules[receiver as usize];

            let mut signal: u8 = match *module_type {
                BROADCASTER => 1,
                FLIPFLOP => {
                    if pulse == 0 {
                        states[receiver as usize] = 1 - states[receiver as usize];
                        states[receiver as usize] + 1
                    } else {
                        0
                    }
                }
                CONJUNCTION => {
                    if parents[receiver as usize]
                        .iter()
                        .all(|parent_idx| last_signals[*parent_idx as usize] == 1)
                    {
                        1
                    } else {
                        2
                    }
                }
                OUTPUT => 0,
                _ => panic!("{}", module_type),
            };

            if signal > 0 {
                signal -= 1;
                last_signals[receiver as usize] = signal;
                for child in children {
                    pulses.push_back((*child, signal));
                    signals[signal as usize] += 1;
                }
            }
        }
    }
    (signals[0] * signals[1]).to_string()
}

pub fn part2(input: &[u8]) -> String {
    let modules = parse(input); // ref?

    let mut parents: Vec<Vec<ModuleIndex>> = vec![Vec::default(); 59];
    for (i, module) in modules.iter().enumerate() {
        for child in &module.1 {
            parents[*child as usize].push(i as ModuleIndex);
        }
    }

    let mut output_parent: ModuleIndex = 0;

    for (i, module) in modules.iter().enumerate() {
        if module.0 == 0 {
            output_parent = parents[i][0];
            break;
        }
    }

    let mut last_signals = vec![0u8; 59];
    let mut states = vec![0u8; 59];

    let mut parent_cycles: HashMap<ModuleIndex, usize> = HashMap::new();

    let mut count = 1;
    'cycle_search: loop {
        let mut pulses: VecDeque<(ModuleIndex, u8)> = VecDeque::new();
        pulses.push_back((0, 0));

        while let Some((receiver, pulse)) = pulses.pop_front() {
            let (module_type, children) = &modules[receiver as usize];

            let signal: Option<u8> = match *module_type {
                BROADCASTER => Some(0),
                FLIPFLOP => {
                    if pulse == 0 {
                        states[receiver as usize] = 1 - states[receiver as usize];
                        Some(states[receiver as usize])
                    } else {
                        None
                    }
                }
                CONJUNCTION => {
                    if parents[receiver as usize]
                        .iter()
                        .all(|parent_idx| last_signals[*parent_idx as usize] == 1)
                    {
                        Some(0)
                    } else {
                        Some(1)
                    }
                }
                OUTPUT => None,
                _ => panic!("{}", module_type),
            };

            if let Some(signal) = signal {
                last_signals[receiver as usize] = signal;
                for child in children {
                    if *child == output_parent && signal == 1 {
                        parent_cycles.entry(receiver).or_insert(count);
                        if parent_cycles.len() == parents[output_parent as usize].len() {
                            break 'cycle_search;
                        }
                    }
                    pulses.push_back((*child, signal));
                }
            }
        }
        count += 1;
    }

    lcm(parent_cycles
        .values()
        .map(|v| *v as usize)
        .collect::<Vec<_>>())
    .to_string()
}

type ModuleIndex = u16;
type Module = (u8, Vec<ModuleIndex>); // (module_type, children)

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(nums: Vec<usize>) -> usize {
    let mut n = nums.iter();
    let mut a = *n.next().unwrap();
    while let Some(&b) = n.next() {
        a = a * b / gcd(a, b);
    }
    a
}
