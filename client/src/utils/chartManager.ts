import {
  LineSeries,
  HistogramSeries,
  ColorType,
  createChart as createLightWeightChart,
  CrosshairMode,
  ISeriesApi,
  UTCTimestamp,
} from 'lightweight-charts';

export class ChartManager {
  private priceSeries: ISeriesApi<'Line'>;
  private volumeSeries: ISeriesApi<'Histogram'>;
  private lastUpdateTime: number = 0;
  private chart: any;
  private currentBar: {
    close: number | null;
    volume: number | null;
  } = {
    close: null,
    volume: null,
  };

  constructor(
    ref: any,
    initialData: any[],
    layout: { background: string; color: string }
  ) {
    console.log('chart data: ', initialData);
    const chart = createLightWeightChart(ref, {
      autoSize: true,
      overlayPriceScales: {
        ticksVisible: true,
        borderVisible: false,
      },
      crosshair: {
        mode: CrosshairMode.Normal,
        vertLine: {
          width: 1,
          color: 'rgba(224, 224, 224, 0.5)',
          style: 2, // dashed
        },
        horzLine: {
          width: 1,
          color: 'rgba(224, 224, 224, 0.5)',
          style: 2, // dashed
        },
      },
      rightPriceScale: {
        visible: true,
        borderVisible: false,
        ticksVisible: true,
        entireTextOnly: true,
      },
      timeScale: {
        borderVisible: false,
        tickMarkFormatter: (time: UTCTimestamp) => {
          const date = new Date(time * 1000);
          return date.toLocaleDateString('en-US', {
            month: 'short',
            day: 'numeric',
          });
        },
      },
      grid: {
        horzLines: {
          color: 'rgba(197, 203, 206, 0.1)',
          visible: true,
        },
        vertLines: {
          color: 'rgba(197, 203, 206, 0.1)',
          visible: true,
        },
      },
      layout: {
        background: {
          type: ColorType.Solid,
          color: layout.background,
        },
        textColor: 'rgba(255, 255, 255, 0.7)',
        fontSize: 12,
      },
    });
    this.chart = chart;

    // Price Series
    this.priceSeries = chart.addSeries(LineSeries, {
      color: '#2962FF', // Vibrant blue
      lineWidth: 2,
      // priceLine: {
      //   color: 'rgba(41, 98, 255, 0.5)',
      //   lineWidth: 1,
      //   lineStyle: 2, // dashed
      // },
      crosshairMarkerVisible: true,
      crosshairMarkerRadius: 4,
      crosshairMarkerBorderColor: 'rgba(41, 98, 255, 1)',
      crosshairMarkerBackgroundColor: 'rgba(41, 98, 255, 1)',
    });
    this.priceSeries.priceScale().applyOptions({
      scaleMargins: {
        top: 0.1,
        bottom: 0.3,
      },
      borderVisible: false,
    });

    // Volume Series
    this.volumeSeries = chart.addSeries(HistogramSeries, {
      color: 'rgba(38, 166, 154, 0.5)', // Soft green with transparency
      priceFormat: {
        type: 'volume',
      },
      priceScaleId: '',
    });
    this.volumeSeries.priceScale().applyOptions({
      scaleMargins: {
        top: 0.9,
        bottom: 0,
      },
      borderVisible: false,
    });

    // Set initial data
    const priceData = initialData.map((data) => ({
      time: Math.floor(data.timestamp / 1000) as UTCTimestamp,
      value: data.close,
    }));

    const volumeData = initialData.map((data, index) => ({
      time: Math.floor(data.timestamp / 1000) as UTCTimestamp,
      value: data.volume,
      color:
        index > 0 && data.close >= initialData[index - 1].close
          ? 'rgba(38, 166, 154, 0.5)' // Green with transparency
          : 'rgba(239, 83, 80, 0.5)', // Red with transparency
    }));

    this.priceSeries.setData(priceData);
    this.volumeSeries.setData(volumeData);

    // Fit content and add margin
    chart.timeScale().fitContent();
  }

  public update(updatedPrice: any) {
    if (!this.lastUpdateTime) {
      this.lastUpdateTime = new Date().getTime();
    }

    // Update price series
    this.priceSeries.update({
      time: (this.lastUpdateTime / 1000) as UTCTimestamp,
      value: updatedPrice.close,
    });

    // Update volume series
    this.volumeSeries.update({
      time: (this.lastUpdateTime / 1000) as UTCTimestamp,
      value: updatedPrice.volume,
      color:
        updatedPrice.close >= (this.currentBar.close || 0)
          ? 'rgba(38, 166, 154, 0.5)' // Green with transparency
          : 'rgba(239, 83, 80, 0.5)', // Red with transparency
    });

    // Update current bar
    this.currentBar = {
      close: updatedPrice.close,
      volume: updatedPrice.volume,
    };

    if (updatedPrice.newCandleInitiated) {
      this.lastUpdateTime = updatedPrice.time;
    }
  }

  public destroy() {
    this.chart.remove();
  }
}
