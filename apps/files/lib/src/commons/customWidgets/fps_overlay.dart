import 'dart:ui';
import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';

class FPSOverlay extends StatefulWidget {
  final Widget child;
  final Alignment alignment;
  final bool visible;
  final bool showChart;

  const FPSOverlay({
    Key? key,
    required this.child,
    this.alignment = Alignment.topRight,
    this.visible = true,
    this.showChart = false,
  }) : super(key: key);

  @override
  State<FPSOverlay> createState() => _FPSOverlayState();
}

class _FPSOverlayState extends State<FPSOverlay> {
  double _currentFPS = 60.0;
  Duration _lastTimestamp = Duration.zero;
  bool _isFirstFrame = true;
  
  // For averaging FPS over multiple frames
  final List<double> _fpsHistory = [];
  static const int _maxHistoryLength = 10;

  @override
  void initState() {
    super.initState();
    if (widget.visible) {
      // Start measuring on the next frame
      SchedulerBinding.instance.addPostFrameCallback(_measureFrame);
    }
  }

  void _measureFrame(Duration timestamp) {
    if (!mounted || !widget.visible) return;

    if (_isFirstFrame) {
      // Skip the first frame as we need a reference point
      _lastTimestamp = timestamp;
      _isFirstFrame = false;
    } else {
      // Calculate the time difference between frames
      final frameDuration = timestamp - _lastTimestamp;
      
      if (frameDuration.inMicroseconds > 0) {
        // Calculate instantaneous FPS
        // FPS = 1,000,000 microseconds per second / frame duration in microseconds
        final instantFPS = 1000000 / frameDuration.inMicroseconds;
        
        // Add to history
        _fpsHistory.add(instantFPS);
        if (_fpsHistory.length > _maxHistoryLength) {
          _fpsHistory.removeAt(0);
        }
        
        // Calculate average FPS for smoother display
        final avgFPS = _fpsHistory.reduce((a, b) => a + b) / _fpsHistory.length;
        
        // Update the display
        if (mounted) {
          setState(() {
            _currentFPS = avgFPS.clamp(0.0, 120.0); // Clamp between 0-120 FPS
          });
        }
      }
      
      _lastTimestamp = timestamp;
    }

    // Schedule the next frame measurement
    SchedulerBinding.instance.addPostFrameCallback(_measureFrame);
  }

  Color _getFPSColor() {
    if (_currentFPS >= 55) return Colors.green;
    if (_currentFPS >= 45) return Colors.yellow;
    if (_currentFPS >= 30) return Colors.orange;
    return Colors.red;
  }

  String _getPerformanceIndicator() {
    if (_currentFPS >= 55) return '●'; // Smooth
    if (_currentFPS >= 45) return '●'; // Good
    if (_currentFPS >= 30) return '●'; // Moderate
    return '●'; // Poor
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        widget.child,
        if (widget.visible)
          Positioned(
            top: widget.alignment == Alignment.topRight || 
                 widget.alignment == Alignment.topLeft ? 50 : null,
            bottom: widget.alignment == Alignment.bottomRight || 
                    widget.alignment == Alignment.bottomLeft ? 16 : null,
            right: widget.alignment == Alignment.topRight || 
                   widget.alignment == Alignment.bottomRight ? 16 : null,
            left: widget.alignment == Alignment.topLeft || 
                  widget.alignment == Alignment.bottomLeft ? 16 : null,
            child: IgnorePointer(
              child: Container(
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 8,
                ),
                decoration: BoxDecoration(
                  color: Colors.black.withOpacity(0.75),
                  borderRadius: BorderRadius.circular(11),
                  border: Border.all(
                    color: _getFPSColor().withOpacity(0.6),
                    width: 2,
                  ),
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withOpacity(0.3),
                      blurRadius: 8,
                      offset: const Offset(0, 2),
                    ),
                  ],
                ),
                child: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text(
                      _getPerformanceIndicator(),
                      style: TextStyle(
                        color: _getFPSColor(),
                        fontSize: 16,
                        height: 1,
                      ),
                    ),
                    const SizedBox(width: 8),
                    Text(
                      '${_currentFPS.toStringAsFixed(1)} FPS',
                      style: TextStyle(
                        color: _getFPSColor(),
                        fontSize: 14,
                        fontWeight: FontWeight.bold,
                        fontFamily: 'monospace',
                        fontFeatures: const [FontFeature.tabularFigures()],
                      ),
                    ),
                    if (widget.showChart) ...[
                      const SizedBox(width: 8),
                      _buildMiniChart(),
                    ],
                  ],
                ),
              ),
            ),
          ),
      ],
    );
  }

  Widget _buildMiniChart() {
    if (_fpsHistory.isEmpty) {
      return const SizedBox.shrink();
    }

    return SizedBox(
      width: 60,
      height: 20,
      child: CustomPaint(
        painter: _FPSChartPainter(
          fpsHistory: _fpsHistory,
          color: _getFPSColor(),
        ),
      ),
    );
  }

  @override
  void dispose() {
    // The post frame callback will automatically stop when disposed
    super.dispose();
  }
}

class _FPSChartPainter extends CustomPainter {
  final List<double> fpsHistory;
  final Color color;

  _FPSChartPainter({
    required this.fpsHistory,
    required this.color,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (fpsHistory.isEmpty) return;

    final paint = Paint()
      ..color = color.withOpacity(0.7)
      ..strokeWidth = 1.5
      ..style = PaintingStyle.stroke;

    final path = Path();
    final maxFPS = 60.0;
    final step = size.width / (fpsHistory.length - 1).clamp(1, double.infinity);

    for (int i = 0; i < fpsHistory.length; i++) {
      final x = i * step;
      final normalizedFPS = (fpsHistory[i] / maxFPS).clamp(0.0, 1.0);
      final y = size.height - (normalizedFPS * size.height);

      if (i == 0) {
        path.moveTo(x, y);
      } else {
        path.lineTo(x, y);
      }
    }

    canvas.drawPath(path, paint);
  }

  @override
  bool shouldRepaint(_FPSChartPainter oldDelegate) {
    return oldDelegate.fpsHistory != fpsHistory || oldDelegate.color != color;
  }
}