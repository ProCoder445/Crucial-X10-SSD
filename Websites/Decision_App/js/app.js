const FORM = document.querySelector("form");
const TIME = FORM.querySelector("#time");
const FOCUS = FORM.querySelector("#focus");
const ACTIVITY = FORM.querySelector("#fav_activity");
const SUBMIT = FORM.querySelector("button");
SUBMIT.disabled = true;
const END = document.querySelector("p");
const IMG = document.querySelector("img");
let recommendation = {
  focus_rec: null,
  activity_rec: null,
  time_rec: null
};


//Error msg if form has not loaded
if(!(FORM && TIME && FOCUS && ACTIVITY))
  alert("Some or all of the form has not loaded, please close to page and retry");


//Allow the user to submit the form if all Qs are answered
FORM.addEventListener("input", () => {
  SUBMIT.disabled = (TIME.value === "" || FOCUS.value === "" || ACTIVITY.value === "");
});

FORM.addEventListener("submit", (event) => {
  event.preventDefault();
  //Determine Focus value
  switch (Number(FOCUS.value)) {
    case 1:
    case 2:
    case 3:
      console.log("rec is games");
      recommendation.focus_rec = "games";
      break;
    case 4:
      console.log("rec is association");
      recommendation.focus_rec = "association";
      break;
    case 5:
      console.log("rec is norm");
      recommendation.focus_rec = "normal";
      break;
    default:
      console.error("default case reached on line 34 of app.js");
      break;
  }
  
  //Determine type
  switch (ACTIVITY.value) {
    case "games":
      console.log("rec is games");
      recommendation.activity_rec = "games";
      break;
    case "studying":
      console.log("rec is normal");
      recommendation.activity_rec = "normal";
      break;
    case "Playing sports":
      console.log("rec is association");
      recommendation.activity_rec = "association";
      break;
    default:
      console.error("default case reached on line 61 of app.js");
      break;
  }

    //Determine time limit
    switch (TIME.value)  {
      case "2h":
      case "1h":
        console.log("rec is norm");
        recommendation.time_rec = "normal";
        break;
      case "30m":
        console.log("rec is association");
        recommendation.time_rec = "association";
        break;
      case "15m":
        console.log("rec is games");
        recommendation.time_rec = "games";
        break;
    }
  let count_games = 0, count_assoc = 0, count_norm = 0;

  //Match values
  switch (recommendation.activity_rec) {
    case "games":
      count_games += 1;
      break;
    case "association":
      count_assoc += 1;
      break;
    case "normal":
      count_norm += 1;
  
  }

  
  switch (recommendation.focus_rec) {
    case "games":
      count_games += 1;
      break;
    case "association":
      count_assoc += 1;
      break;
    case "normal":
      count_norm += 1;
  
  }


  switch (recommendation.time_rec) {
    case "games":
      count_games += 1;
      break;
    case "association":
      count_assoc += 1;
      break;
    case "normal":
      count_norm += 1;
  
  }



  if (count_assoc > count_games && count_assoc > count_norm) {
    END.innerText = "You should use association to help you study!";
    IMG.src = "/img/association.png";
  } else if (count_games > count_assoc && count_games > count_norm) {
    END.innerText = "You should study using games!";
    IMG.src = "/img/game.png";
  } else if (count_norm > count_games && count_norm > count_assoc) {
    END.innerText = "You should continue studying normally!";
    IMG.src = "/img/normal.png";
  } else {
    END.innerText = "You should try game, association, and normal studying to see which fits you best!";
    IMG.src = "/img/all_three.png";
  }
});