import LineGraph from "./line";
import BarGraph from "./Bar";
import '../App.css';

export default function Etiquetas() {
  return (
    <div>
      <h1>Etiquetas</h1>
      <div className="charts_area">
        <div className="line-graph">
          <LineGraph />
        </div>
        <div className="bar-graph">
          <BarGraph/>
        </div>
      </div>
    </div>
  );
}