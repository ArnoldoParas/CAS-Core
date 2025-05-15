import { Bar } from 'react-chartjs-2';

import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    BarElement,
    Title,
    Tooltip,
    Legend,
} from 'chart.js';

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

export const options = {
    indexAxis: 'y',
    elements: {
      bar: {
        borderWidth: 2,
      },
    },
    responsive: true,
    maintainAspectRatio: false,
    plugins: {
      legend: {
        position: 'right',
        onClick: () => {},
      },
      title: {
        display: true,
        text: 'Top 5: Próximas Visitas',
        font: {
            size: 20,
        },
      },
    },
};

const labels = ['FIME', 'Prepa 2', 'FACPyA', 'Sindicato', 'Prepa 17'];

export const data = {
  labels,
  datasets: [
    {
      label: 'Dias',
      data: [20, 120, 76, 34, 200],
      borderColor: 'rgb(255, 99, 132)',
      backgroundColor: 'rgba(255, 99, 132, 0.5)',
    },
  ],
};
  
export default function BarGraph() {
  return <Bar options={options} data={data} />;
}