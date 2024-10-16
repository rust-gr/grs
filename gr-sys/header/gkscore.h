#ifndef _GKSCORE_H_
#define _GKSCORE_H_

typedef char size_t; // placeholder-type (dropped by rust-bindgen)

#ifndef DLLEXPORT
#define DLLEXPORT
#endif

#ifdef __cplusplus
extern "C" {
#endif

#define FEPS 1.0E-09

#define GRALGKS 3
#define GLIGKS 4
#define GKS5 5

#define MAX_WS 16      /* maximum number of workstations */
#define MAX_TNR 9      /* maximum number of normalization transformations */
#define MAX_COLOR 1256 /* maximum number of predefined colors */

#define FIX_COLORIND(c) (c) < 0 ? 0 : (c) < MAX_COLOR ? (c) : MAX_COLOR - 1

#define OPEN_GKS 0
#define CLOSE_GKS 1
#define OPEN_WS 2
#define CLOSE_WS 3
#define ACTIVATE_WS 4
#define DEACTIVATE_WS 5
#define CLEAR_WS 6
#define REDRAW_SEG_ON_WS 7
#define UPDATE_WS 8
#define SET_DEFERRAL_STATE 9
#define MESSAGE 10
#define ESCAPE 11
#define POLYLINE 12
#define POLYMARKER 13
#define TEXT 14
#define FILLAREA 15
#define CELLARRAY 16
#define GDP 17
#define SET_PLINE_INDEX 18
#define SET_PLINE_LINETYPE 19
#define SET_PLINE_LINEWIDTH 20
#define SET_PLINE_COLOR_INDEX 21
#define SET_PMARK_INDEX 22
#define SET_PMARK_TYPE 23
#define SET_PMARK_SIZE 24
#define SET_PMARK_COLOR_INDEX 25
#define SET_TEXT_INDEX 26
#define SET_TEXT_FONTPREC 27
#define SET_TEXT_EXPFAC 28
#define SET_TEXT_SPACING 29
#define SET_TEXT_COLOR_INDEX 30
#define SET_TEXT_HEIGHT 31
#define SET_TEXT_UPVEC 32
#define SET_TEXT_PATH 33
#define SET_TEXT_ALIGN 34
#define SET_FILL_INDEX 35
#define SET_FILL_INT_STYLE 36
#define SET_FILL_STYLE_INDEX 37
#define SET_FILL_COLOR_INDEX 38
#define SET_ASF 41
#define SET_COLOR_REP 48
#define SET_WINDOW 49
#define SET_VIEWPORT 50
#define SELECT_XFORM 52
#define SET_CLIPPING 53
#define SET_WS_WINDOW 54
#define SET_WS_VIEWPORT 55
#define CREATE_SEG 56
#define CLOSE_SEG 57
#define DELETE_SEG 58
#define ASSOC_SEG_WITH_WS 61
#define COPY_SEG_TO_WS 62
#define SET_SEG_XFORM 64
#define INITIALIZE_LOCATOR 69
#define REQUEST_LOCATOR 81
#define REQUEST_STROKE 82
#define REQUEST_CHOICE 84
#define REQUEST_STRING 86
#define GET_ITEM 102
#define READ_ITEM 103
#define INTERPRET_ITEM 104
#define EVAL_XFORM_MATRIX 105
#define SET_ENCODING 106
#define INQ_ENCODING 107
#define SET_RESAMPLE_METHOD 108
#define SET_RESIZE_BEHAVIOUR 109
#define INQ_TEXT 110

#define SET_TEXT_SLANT 200
#define DRAW_IMAGE 201
#define SET_SHADOW 202
#define SET_TRANSPARENCY 203
#define SET_COORD_XFORM 204
#define CONFIGURE_WS 205
#define SET_BORDER_WIDTH 206
#define SET_BORDER_COLOR_INDEX 207
#define SELECT_CLIP_XFORM 208
#define INQ_WS_STATE 209
#define SAMPLE_LOCATOR 210
#define SET_CLIP_REGION 211
#define SET_CLIP_SECTOR 212

#define BEGIN_SELECTION 250
#define END_SELECTION 251
#define MOVE_SELECTION 252
#define RESIZE_SELECTION 253
#define INQ_BBOX 254

#define GKS_SET_BBOX_CALLBACK 260
#define GKS_CANCEL_BBOX_CALLBACK 261

#define ENCODING_LATIN1 300
#define ENCODING_UTF8 301

#define MAX_ATTRIBUTE_FCTID SET_WS_VIEWPORT /* maximum function ID for setting an attribute */

typedef struct
{
  int lindex;
  int ltype;
  double lwidth;
  int plcoli;
  int mindex;
  int mtype;
  double mszsc;
  int pmcoli;
  int tindex;
  int txfont, txprec;
  double chxp;
  double chsp;
  int txcoli;
  double chh;
  double chup[2];
  int txp;
  int txal[2];
  int findex;
  int ints;
  int styli;
  int facoli;
  double window[MAX_TNR][4], viewport[MAX_TNR][4];
  int cntnr, clip, opsg;
  double mat[3][2];
  int asf[13];
  int wiss, version;
  int fontfile;
  int input_encoding;
  double txslant;
  double shoff[2];
  double blur;
  double alpha;
  double a[MAX_TNR], b[MAX_TNR], c[MAX_TNR], d[MAX_TNR];
  unsigned int resample_method;
  double bwidth;
  int bcoli;
  int clip_tnr;
  int clip_region;
  double clip_start_angle, clip_end_angle;
  int resize_behaviour;
  double aspect_ratio;
  char *(*callback)(const char *);
  int debug;
} gks_state_list_t;

typedef struct
{
  int wkid;
  char *path;
  int wtype;
  int conid;
  void *ptr;
  double vp[4];
  char *name;
} ws_list_t;

typedef struct
{
  int wtype;
  int dcunit;
  double sizex, sizey;
  int unitsx, unitsy;
  int wscat;
  char *type;
  char *env;
  char *name;
} ws_descr_t;

typedef struct
{
  int state;
  char *buffer;
  int size, nbytes, position;
  int empty;
} gks_display_list_t;

typedef struct
{
  int width;
  int height;
  double device_pixel_ratio;
} gks_ws_state_t;

typedef struct
{
  double x;
  double y;
  int status;
} gks_locator_t;

DLLEXPORT char *gks_malloc(int size);
DLLEXPORT char *gks_realloc(void *ptr, int size);
DLLEXPORT void gks_free(void *ptr);
DLLEXPORT char *gks_strdup(const char *str);

DLLEXPORT void gks_perror(const char *, ...);

DLLEXPORT void gks_resample(const unsigned char *source_image, unsigned char *target_image, size_t source_width,
                            size_t source_height, size_t target_width, size_t target_height, size_t stride, int swapx,
                            int swapy, unsigned int resample_method);
DLLEXPORT const char *gks_getenv(const char *env);

DLLEXPORT void gks_dl_write_item(gks_display_list_t *d, int fctid, int dx, int dy, int dimx, int *ia, int lr1,
                                 double *r1, int lr2, double *r2, int lc, char *c, gks_state_list_t *gkss);
DLLEXPORT int gks_dl_read_item(char *dl, gks_state_list_t **gkss,
                               void (*fn)(int fctid, int dx, int dy, int dimx, int *ia, int lr1, double *r1, int lr2,
                                          double *r2, int lc, char *chars, void **ptr));

#ifndef EMSCRIPTEN

DLLEXPORT void gks_x11_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                              int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_gs_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                             int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_gtk_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                              int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_wx_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                             int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_qt_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                             int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_svg_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                              int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_wmf_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                              int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_quartz_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                                 int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_gl_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                             int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_cairo_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                                int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_zmq_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                              int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_pgf_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                              int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_video_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                                int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

DLLEXPORT void gks_agg_plugin(int fctid, int dx, int dy, int dimx, int *i_arr, int len_f_arr_1, double *f_arr_1,
                              int len_f_arr_2, double *f_arr_2, int len_c_arr, char *c_arr, void **ptr);

#else

#endif

DLLEXPORT int gks_ft_get_metrics(int font, double fontsize, unsigned int codepoint, unsigned int dpi, double *width,
                                 double *height, double *depth, double *advance, double *bearing, double *xmin,
                                 double *xmax, double *ymin, double *ymax);
DLLEXPORT double gks_ft_get_kerning(int font, double fontsize, unsigned int dpi, unsigned int first_codepoint,
                                    unsigned int second_codepoint);
DLLEXPORT void gks_ft_text3d(double x, double y, double z, char *text, int axis, gks_state_list_t *gkss,
                             double heightFactor, double *scaleFactors,
                             void (*gdp)(int, double *, double *, int, int, int *),
                             void (*wc3towc)(double *, double *, double *));
DLLEXPORT void gks_ft_inq_text3d_extent(double x, double y, double z, char *text, int axis, gks_state_list_t *gkss,
                                        double heightFactor, double *scaleFactors,
                                        void (*gdp)(int, double *, double *, int, int, int *),
                                        void (*wc3towc)(double *, double *, double *), double *bx, double *by);
DLLEXPORT void gks_ft_set_bearing_x_direction(int);
DLLEXPORT void gks_ft_inq_bearing_x_direction(int *);
DLLEXPORT int gks_ft_load_user_font(char *font, int ignore_file_not_found);

DLLEXPORT void gks_set_encoding(int encoding);
DLLEXPORT void gks_inq_encoding(int *encoding);

DLLEXPORT void gks_set_callback(char *(*callback)(const char *));

#ifdef __cplusplus
}
#endif

#endif
