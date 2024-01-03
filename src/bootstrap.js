import init, * as wasm from '../pkg/unrailed_seed_analyzer.js';

async function run() {
  await init();
}

run();
window.wasm = wasm;


//terrain selector
const terrain_list = ["plain", "dessert", "snow", "hell", "space", "mars"]
var cur_terrain_selection = [true, true, false, true, false, false]

function update_terrain_prob() {
  let terrain_prob = document.getElementById("terrain_prob");
  let selected_cnt = cur_terrain_selection.filter(x => x).length;
  if (selected_cnt < 3){
    terrain_prob.innerHTML = "0%";
    return;
  }
  let prob = math.combinations(selected_cnt, 3) / math.combinations(terrain_list.length, 3);
  terrain_prob.innerHTML = prob * 100 + "%";
}
function terrain_selector(){
  let terrain_selector_title = document.createElement("div");
  terrain_selector_title.classList.add("terrain_selector_title");
  terrain_selector_title.innerHTML = "<h2>Terrain Selector</h2><p id='terrain_prob' class='prob_display'></p>";
  document.body.appendChild(terrain_selector_title);
  update_terrain_prob();
  let terrain_selector_container = document.createElement("div");
  terrain_selector_container.classList.add("terrain_selector_container");
  //terrain_selector_container.style.minWidth = "" + terrain_list.length*70 + "px";
  for (const i in terrain_list){
    let terrain = terrain_list[i];
    let terrain_selection = document.createElement("div");
    terrain_selection.classList.add("terrain_selection");
    let button_overlay = document.createElement("div");
    button_overlay.classList.add("terrain_selection_overlay");
    if (cur_terrain_selection[i] === true){
      button_overlay.classList.add("selected_overlay");
    }else{
      button_overlay.classList.add("forbidden_overlay");
    }
    button_overlay.onclick = (ev) => {
      ev.preventDefault();
      if (cur_terrain_selection[i] === true){
        button_overlay.classList.remove("selected_overlay");
        button_overlay.classList.add("forbidden_overlay");
        cur_terrain_selection[i] = false;
      }else{
        button_overlay.classList.remove("forbidden_overlay");
        button_overlay.classList.add("selected_overlay");
        cur_terrain_selection[i] = true;
      }
      update_terrain_prob();
    }
    button_overlay.addEventListener("contextmenu", button_overlay.onclick);
    terrain_selection.appendChild(button_overlay);
    let terrain_picture = document.createElement("img");
    terrain_picture.setAttribute("src", "./assets/" + terrain + "_terrain.png");
    terrain_selection.append(terrain_picture);
    terrain_selector_container.appendChild(terrain_selection);
  }
  document.body.appendChild(terrain_selector_container);
}

var strict_wagon_order = true;
var max_wagon_lookup_cnt = 20;
var selected_wagons = ["dynamite", "supercharger", "dynamite", "collector"];


function update_wagon_prob(){
  //TODO
}
function create_wagon_selector(){
  //strict order?
  //max cnt?
  let wagon_selector_title = document.createElement("div");
  wagon_selector_title.classList.add("wagon_selector_title");
  wagon_selector_title.innerHTML = "<h2>Wagon Selector</h2><p id='wagon_prob' class='prob_display'></p>";
  document.body.appendChild(wagon_selector_title);
  update_wagon_prob();
  let wagon_selector_container = document.createElement("div");
  wagon_selector_container.classList.add("wagon_selector_container");
  wagon_selector_container.id = "wagon_selector_container";
  let display_wagons = selected_wagons.reverse();
  var wagon_map = [];
  if (strict_wagon_order === true){
    for (const wagon of display_wagons){
        wagon_map.push([wagon, 1]);
    }
  }else{
    for (const wagon of display_wagons){
      if (wagon_map.map(x => x[0]).indexOf(wagon) > -1){
        wagon_map.find(x => x[0] === wagon)[1] += 1;
      }else{
        wagon_map.push([wagon, 1]);
      }
    }
  }
  for (const [wagon, cnt] of wagon_map) {
    let wagon_selection = document.createElement("div");
    wagon_selection.classList.add("wagon_selection");
    let button_overlay = document.createElement("div");
    button_overlay.classList.add("wagon_selection_overlay");
    wagon_selection.appendChild(button_overlay);
    let wagon_picture = document.createElement("img");
    wagon_picture.setAttribute("src","./assets/" + wagon + "_wagon.png");
    wagon_selection.append(wagon_picture);
    wagon_selection.draggable = true;
    wagon_selection.addEventListener("dragstart", (ev) => {
      let wagon_selections = document.querySelectorAll(".wagon_selection");
      for (const selection of wagon_selections) {
        selection.style.borderRadius = "5px";
      }
      setTimeout(() =>{
        wagon_selection.hidden = true;
      }, 0);
    });
    wagon_selection.addEventListener("dragend", (ev) => {
      wagon_selection.hidden = false;
      let wagon_selections = document.querySelectorAll(".wagon_selection");
      for (const selection of wagon_selections) {
        selection.style.borderRadius = "0px";
      }
    });
    wagon_selector_container.appendChild(wagon_selection);
  }
  document.body.appendChild(wagon_selector_container);
}

terrain_selector();
create_wagon_selector();